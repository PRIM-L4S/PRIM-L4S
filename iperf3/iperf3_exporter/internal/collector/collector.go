// Copyright 2019 Edgard Castro
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Package collector provides the Prometheus collector for iperf3 metrics.
package collector

import (
	"context"
	"log/slog"
	"strconv"
	"sync"
	"time"

	"github.com/edgard/iperf3_exporter/internal/config"
	"github.com/edgard/iperf3_exporter/internal/iperf"
	"github.com/prometheus/client_golang/prometheus"
)

const (
	namespace = "iperf3"
)

// Metrics about the iperf3 exporter itself.
var (
	IperfDuration = prometheus.NewSummary(
		prometheus.SummaryOpts{
			Name: prometheus.BuildFQName(namespace, "exporter", "duration_seconds"),
			Help: "Duration of collections by the iperf3 exporter.",
		},
	)
	IperfErrors = prometheus.NewCounter(
		prometheus.CounterOpts{
			Name: prometheus.BuildFQName(namespace, "exporter", "errors_total"),
			Help: "Errors raised by the iperf3 exporter.",
		},
	)
)

type Mode int

const (
	TCP Mode = iota
	UDP
	Both
)

// ProbeConfig represents the configuration for a single probe.
type ProbeConfig struct {
	Target      string
	Port        int
	Period      time.Duration
	Timeout     time.Duration
	ReverseMode bool
	Mode        Mode
	Bitrate     string
}

// Collector implements the prometheus.Collector interface for iperf3 metrics.
type Collector struct {
	target  string
	port    int
	period  time.Duration
	timeout time.Duration
	mutex   sync.RWMutex
	reverse bool
	mode    Mode
	bitrate string
	logger  *slog.Logger
	runner  iperf.Runner

	// Metrics
	up                 *prometheus.Desc
	sentSeconds        *prometheus.Desc
	sentBytes          *prometheus.Desc
	sentBytesTotal     *prometheus.CounterVec
	receivedSeconds    *prometheus.Desc
	receivedBytes      *prometheus.Desc
	receivedBytesTotal *prometheus.CounterVec
	// TCP-specific metrics
	retransmits *prometheus.Desc
	minRtt      *prometheus.Desc
	maxRtt      *prometheus.Desc
	meanRtt     *prometheus.Desc
	// UDP-specific metrics
	sentPackets     *prometheus.Desc
	sentJitter      *prometheus.Desc
	sentLostPackets *prometheus.Desc
	sentLostPercent *prometheus.Desc
	recvPackets     *prometheus.Desc
	recvJitter      *prometheus.Desc
	recvLostPackets *prometheus.Desc
	recvLostPercent *prometheus.Desc
}

func (mode Mode) ToIperf() iperf.TCPUDPMode {
	if mode == TCP {
		return iperf.TCP
	} else if mode == UDP {
		return iperf.UDP
	} else {
		panic("ToIperf got an invalid mode")
	}
}

// NewCollector creates a new Collector for iperf3 metrics.
func NewCollector(logger *slog.Logger) *Collector {
	return NewCollectorWithRunner(logger, iperf.NewRunner(logger))
}

// NewCollectorWithRunner creates a new Collector for iperf3 metrics with a custom runner.
func NewCollectorWithRunner(logger *slog.Logger, runner iperf.Runner) *Collector {
	// Common labels for all metrics
	labels := []string{"target", "port", "version"}

	return &Collector{
		// the config is initialized to zero-value, and updated before each run
		logger: logger,
		runner: runner,

		// Define metrics with labels
		up: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "up"),
			"Was the last iperf3 probe successful (1 for success, 0 for failure).",
			labels, nil,
		),
		sentSeconds: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "sent_seconds"),
			"Total seconds spent sending packets.",
			labels, nil,
		),
		sentBytes: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "sent_bytes"),
			"Total sent bytes for the last test run.",
			labels, nil,
		),
		sentBytesTotal: prometheus.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: namespace,
				Name:      "sent_bytes_total",
				Help:      "Total sent bytes counter over all test runs.",
			},
			labels,
		),
		receivedSeconds: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "received_seconds"),
			"Total seconds spent receiving packets.",
			labels, nil,
		),
		receivedBytes: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "received_bytes"),
			"Total received bytes for the last test run.",
			labels, nil,
		),
		receivedBytesTotal: prometheus.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: namespace,
				Name:      "received_bytes_total",
				Help:      "Total received bytes counter over all test runs.",
			},
			labels,
		),
		// TCP-specific metrics
		retransmits: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "retransmits"),
			"Total retransmits for the last test run.",
			labels, nil,
		),
		minRtt: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "min_rtt"),
			"Minimum Round-trip time measured in the last test in microseconds.",
			labels, nil,
		),
		maxRtt: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "max_rtt"),
			"Maximum Round-trip time measured in the last test in microseconds.",
			labels, nil,
		),
		meanRtt: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "mean_rtt"),
			"Mean Round-trip time measured in the last test in microseconds.",
			labels, nil,
		),
		// UDP-specific metrics
		sentPackets: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "sent_packets"),
			"Total sent packets for the last UDP test run.",
			labels, nil,
		),
		sentJitter: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "sent_jitter_ms"),
			"Jitter in milliseconds for sent packets in UDP mode.",
			labels, nil,
		),
		sentLostPackets: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "sent_lost_packets"),
			"Total lost packets from the sender in the last UDP test run.",
			labels, nil,
		),
		sentLostPercent: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "sent_lost_percent"),
			"Percentage of packets lost from the sender in the last UDP test run.",
			labels, nil,
		),
		recvPackets: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "received_packets"),
			"Total received packets for the last UDP test run.",
			labels, nil,
		),
		recvJitter: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "received_jitter_ms"),
			"Jitter in milliseconds for received packets in UDP mode.",
			labels, nil,
		),
		recvLostPackets: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "received_lost_packets"),
			"Total lost packets at the receiver in the last UDP test run.",
			labels, nil,
		),
		recvLostPercent: prometheus.NewDesc(
			prometheus.BuildFQName(namespace, "", "received_lost_percent"),
			"Percentage of packets lost at the receiver in the last UDP test run.",
			labels, nil,
		),
	}
}

func (c *Collector) UpdateConfig(config ProbeConfig) {
	c.target = config.Target
	c.port = config.Port
	c.period = config.Period
	c.timeout = config.Timeout
	c.reverse = config.ReverseMode
	c.mode = config.Mode
	c.bitrate = config.Bitrate
}

// Describe implements the prometheus.Collector interface.
func (c *Collector) Describe(ch chan<- *prometheus.Desc) {
	ch <- c.up
	ch <- c.sentSeconds
	ch <- c.sentBytes
	c.sentBytesTotal.Describe(ch)
	ch <- c.receivedSeconds
	ch <- c.receivedBytes
	c.receivedBytesTotal.Describe(ch)

	// TCP-specific metrics
	ch <- c.retransmits

	// UDP-specific metrics
	ch <- c.sentPackets
	ch <- c.sentJitter
	ch <- c.sentLostPackets
	ch <- c.sentLostPercent
	ch <- c.recvPackets
	ch <- c.recvJitter
	ch <- c.recvLostPackets
	ch <- c.recvLostPercent
}

// Collect implements the prometheus.Collector interface.
func (c *Collector) Collect(ch chan<- prometheus.Metric) {
	c.mutex.Lock() // To protect metrics from concurrent collects.
	defer c.mutex.Unlock()

	// Create context with timeout
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	// Run iperf3 test
	var result iperf.Result
	if c.mode == TCP || c.mode == UDP {
		result = c.runner.Run(ctx, iperf.Config{
			Target:      c.target,
			Port:        c.port,
			Period:      c.period,
			Timeout:     c.timeout,
			ReverseMode: c.reverse,
			Mode:        c.mode.ToIperf(),
			Bitrate:     c.bitrate,
			Logger:      c.logger,
		})
	} else {
		resultTCP := c.runner.Run(ctx, iperf.Config{
			Mode:        iperf.TCP,
			Period:      c.period / 2,
			Target:      c.target,
			Port:        c.port,
			Timeout:     c.timeout,
			ReverseMode: c.reverse,
			Bitrate:     c.bitrate,
			Logger:      c.logger,
		})

		time.Sleep(500 * time.Millisecond)

		resultUDP := c.runner.Run(ctx, iperf.Config{
			Mode:        iperf.UDP,
			Period:      c.period / 2,
			Target:      c.target,
			Port:        c.port,
			Timeout:     c.timeout,
			ReverseMode: c.reverse,
			Bitrate:     c.bitrate,
			Logger:      c.logger,
		})

		result = iperf.Result{
			// Common
			Success:               resultTCP.Success && resultUDP.Success,
			SentSeconds:           resultTCP.SentSeconds + resultUDP.SentSeconds,
			SentBytes:             resultTCP.SentBytes + resultUDP.SentBytes,
			SentBitsPerSecond:     (resultTCP.SentBitsPerSecond + resultUDP.SentBitsPerSecond) / 2,
			ReceivedSeconds:       resultTCP.ReceivedSeconds + resultUDP.ReceivedSeconds,
			ReceivedBytes:         resultTCP.ReceivedBytes + resultUDP.ReceivedBytes,
			ReceivedBitsPerSecond: (resultTCP.ReceivedBitsPerSecond + resultUDP.ReceivedBitsPerSecond) / 2,
			// TCP-specific
			Retransmits: resultTCP.Retransmits,
			MinRtt:      resultTCP.MinRtt,
			MaxRtt:      resultTCP.MaxRtt,
			MeanRtt:     resultTCP.MeanRtt,
			// UDP-specific
			SentPackets:         resultUDP.SentPackets,
			SentJitter:          resultUDP.SentJitter,
			SentLostPackets:     resultUDP.SentLostPackets,
			SentLostPercent:     resultUDP.SentLostPercent,
			ReceivedPackets:     resultUDP.ReceivedPackets,
			ReceivedJitter:      resultUDP.ReceivedJitter,
			ReceivedLostPackets: resultUDP.ReceivedLostPackets,
			ReceivedLostPercent: resultUDP.ReceivedLostPercent,
		}
	}

	// Common label values for all metrics
	labelValues := []string{c.target, strconv.Itoa(c.port), config.AppVersion}

	if !result.Success {
		// In case of failure, we do not display anything
		// This can lead for some throughput to not be displayed, in the case of the 'both' mode.
		IperfErrors.Inc()
		ch <- prometheus.MustNewConstMetric(c.up, prometheus.GaugeValue, 0, labelValues...)
		return
	}

	// Set metrics based on result
	ch <- prometheus.MustNewConstMetric(c.up, prometheus.GaugeValue, 1, labelValues...)
	ch <- prometheus.MustNewConstMetric(c.sentSeconds, prometheus.GaugeValue, result.SentSeconds, labelValues...)
	ch <- prometheus.MustNewConstMetric(c.sentBytes, prometheus.GaugeValue, result.SentBytes, labelValues...)
	c.sentBytesTotal.WithLabelValues(labelValues...).Add(result.SentBytes)
	ch <- prometheus.MustNewConstMetric(c.receivedSeconds, prometheus.GaugeValue, result.ReceivedSeconds, labelValues...)
	ch <- prometheus.MustNewConstMetric(c.receivedBytes, prometheus.GaugeValue, result.ReceivedBytes, labelValues...)
	c.receivedBytesTotal.WithLabelValues(labelValues...).Add(result.ReceivedBytes)

	// Retransmits is only relevant in TCP mode
	if c.mode == TCP || c.mode == Both {
		ch <- prometheus.MustNewConstMetric(c.retransmits, prometheus.GaugeValue, result.Retransmits, labelValues...)
		ch <- prometheus.MustNewConstMetric(c.minRtt, prometheus.GaugeValue, result.MinRtt, labelValues...)
		ch <- prometheus.MustNewConstMetric(c.maxRtt, prometheus.GaugeValue, result.MaxRtt, labelValues...)
		ch <- prometheus.MustNewConstMetric(c.meanRtt, prometheus.GaugeValue, result.MeanRtt, labelValues...)
	}

	// Include UDP-specific metrics when in UDP mode
	if c.mode == UDP || c.mode == Both {
		ch <- prometheus.MustNewConstMetric(c.sentPackets, prometheus.GaugeValue, result.SentPackets, labelValues...)
		ch <- prometheus.MustNewConstMetric(c.sentJitter, prometheus.GaugeValue, result.SentJitter, labelValues...)
		ch <- prometheus.MustNewConstMetric(c.sentLostPackets, prometheus.GaugeValue, result.SentLostPackets, labelValues...)
		ch <- prometheus.MustNewConstMetric(c.sentLostPercent, prometheus.GaugeValue, result.SentLostPercent, labelValues...)
		ch <- prometheus.MustNewConstMetric(c.recvPackets, prometheus.GaugeValue, result.ReceivedPackets, labelValues...)
		ch <- prometheus.MustNewConstMetric(c.recvJitter, prometheus.GaugeValue, result.ReceivedJitter, labelValues...)
		ch <- prometheus.MustNewConstMetric(c.recvLostPackets, prometheus.GaugeValue, result.ReceivedLostPackets, labelValues...)
		ch <- prometheus.MustNewConstMetric(c.recvLostPercent, prometheus.GaugeValue, result.ReceivedLostPercent, labelValues...)
	}

	c.sentBytesTotal.Collect(ch)
	c.receivedBytesTotal.Collect(ch)
}
