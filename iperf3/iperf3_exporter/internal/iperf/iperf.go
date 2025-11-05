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

// Package iperf provides functionality for running iperf3 tests and parsing results.
package iperf

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"log/slog"
	"os/exec"
	"regexp"
	"time"
)

// execCommand is a variable that allows tests to mock exec.Command.
var execCommand = exec.Command

// execCommandContext is a variable that allows tests to mock exec.CommandContext.
var execCommandContext = exec.CommandContext

// lookPath is a variable that allows tests to mock exec.LookPath.
var lookPath = exec.LookPath

// ResetExecCommand resets the execCommand variables to the default implementation.
func ResetExecCommand() {
	execCommand = exec.Command
	execCommandContext = exec.CommandContext
}

// Runner defines the interface for running iperf3 tests.
type Runner interface {
	Run(ctx context.Context, cfg Config) Result
}

// DefaultRunner is the default implementation of the Runner interface.
type DefaultRunner struct {
	Logger *slog.Logger
}

// NewRunner creates a new default iperf3 runner.
func NewRunner(logger *slog.Logger) Runner {
	return &DefaultRunner{
		Logger: logger,
	}
}

// Result represents the parsed result from an iperf3 test.
type Result struct {
	Success               bool
	SentSeconds           float64
	SentBytes             float64
	SentBitsPerSecond     float64
	ReceivedSeconds       float64
	ReceivedBytes         float64
	ReceivedBitsPerSecond float64
	// TCP-specific fields
	Retransmits float64
	MinRtt      float64
	MaxRtt      float64
	MeanRtt     float64
	// UDP-specific fields
	SentPackets         float64
	SentJitter          float64
	SentLostPackets     float64
	SentLostPercent     float64
	ReceivedPackets     float64
	ReceivedJitter      float64
	ReceivedLostPackets float64
	ReceivedLostPercent float64
}

// rawResult collects the partial result from the iperf3 run.
type rawResult struct {
	Start struct {
		TestStart struct {
			Protocol string `json:"protocol"`
		} `json:"test_start"`
	} `json:"start"`
	End struct {
		// TCP mode uses these fields
		SumSent struct {
			Seconds       float64 `json:"seconds"`
			Bytes         float64 `json:"bytes"`
			BitsPerSecond float64 `json:"bits_per_second"`
			Retransmits   float64 `json:"retransmits"`
		} `json:"sum_sent"`
		SumReceived struct {
			Seconds       float64 `json:"seconds"`
			Bytes         float64 `json:"bytes"`
			BitsPerSecond float64 `json:"bits_per_second"`
		} `json:"sum_received"`

		Streams []struct {
			// UDP mode specific structure
			UDP UDPInfo `json:"udp"`
			// TCP mode specific structure
			TCPSender   TCPSender `json:"sender"`
			TCPReceiver TCPSender `json:"receiver"`
		} `json:"streams"`
		// UDP mode specific structure
		Sum UDPInfo `json:"sum"`
	} `json:"end"`
}

// UDPInfo contains the UDP specific metrics
type UDPInfo struct {
	Socket        int     `json:"socket,omitempty"`
	Start         float64 `json:"start,omitempty"`
	End           float64 `json:"end,omitempty"`
	Seconds       float64 `json:"seconds,omitempty"`
	Bytes         float64 `json:"bytes,omitempty"`
	BitsPerSecond float64 `json:"bits_per_second,omitempty"`
	JitterMs      float64 `json:"jitter_ms,omitempty"`
	LostPackets   float64 `json:"lost_packets,omitempty"`
	Packets       float64 `json:"packets,omitempty"`
	LostPercent   float64 `json:"lost_percent,omitempty"`
	Sender        bool    `json:"sender,omitempty"`
}

type TCPSender struct {
	Socket        int     `json:"socket,omitempty"`
	Start         float64 `json:"start,omitempty"`
	End           float64 `json:"end,omitempty"`
	Seconds       float64 `json:"seconds,omitempty"`
	Bytes         float64 `json:"bytes,omitempty"`
	BitsPerSecond float64 `json:"bits_per_second,omitempty"`
	Retransmits   float64 `json:"retransmits,omitempty"`
	MaxSndCwnd    float64 `json:"max_snd_cwnd,omitempty"`
	MaxRtt        float64 `json:"max_rtt,omitempty"`
	MinRtt        float64 `json:"min_rtt,omitempty"`
	MeanRtt       float64 `json:"mean_rtt,omitempty"`
	Sender        bool    `json:"sender,omitempty"`
}

type TCPReceiver struct {
	Socket        int     `json:"socket,omitempty"`
	Start         float64 `json:"start,omitempty"`
	End           float64 `json:"end,omitempty"`
	Seconds       float64 `json:"seconds,omitempty"`
	Bytes         float64 `json:"bytes,omitempty"`
	BitsPerSecond float64 `json:"bits_per_second,omitempty"`
	Sender        bool    `json:"sender,omitempty"`
}

// Config represents the configuration for an iperf3 test.
type TCPUDPMode int

const (
	TCP TCPUDPMode = iota
	UDP
)

type Config struct {
	Target      string
	Port        int
	Period      time.Duration
	Timeout     time.Duration
	ReverseMode bool
	Mode        TCPUDPMode
	Bitrate     string
	Logger      *slog.Logger
}

var bitratePattern = regexp.MustCompile(`^[0-9]+(\.[0-9]+)?([KMG])?(\/[0-9]+)?$`)

// ValidateBitrate validates the bitrate format.
func ValidateBitrate(bitrate string) bool {
	if bitrate == "" {
		return true
	}

	return bitratePattern.MatchString(bitrate)
}

func (mode TCPUDPMode) ToString() string {
	switch mode {
	case TCP:
		return "TCP"
	case UDP:
		return "UDP"
	}

	panic("Could not transform to string, unknown TCPUDPMode.")
}

// Run executes an iperf3 test with the given configuration and returns the parsed results.
// This is a convenience function that uses the DefaultRunner.
func Run(ctx context.Context, cfg Config) Result {
	runner := NewRunner(cfg.Logger)

	return runner.Run(ctx, cfg)
}

// Run executes an iperf3 test with the given configuration and returns the parsed results.
func (r *DefaultRunner) Run(ctx context.Context, cfg Config) Result {
	// Create a result with default values
	result := Result{
		Success: false,
	}

	// Validate bitrate if provided
	if cfg.Bitrate != "" && !ValidateBitrate(cfg.Bitrate) {
		cfg.Logger.Error("Invalid bitrate format", "bitrate", cfg.Bitrate)

		return result
	}

	iperfArgs := cfg.generateIperfArgs()

	// Create command with context
	// #nosec G204 - GetIperfCmd returns a hardcoded string and iperfArgs are validated
	var cmd *exec.Cmd
	if ctx != nil {
		// Use the mockable execCommandContext for context-aware commands
		cmd = execCommandContext(ctx, GetIperfCmd(), iperfArgs...)
	} else {
		cmd = execCommand(GetIperfCmd(), iperfArgs...)
	}

	var stderr bytes.Buffer
	cmd.Stderr = &stderr

	// Execute the command
	cfg.Logger.Debug("Running iperf3 command",
		"target", cfg.Target,
		"port", cfg.Port,
		"period", cfg.Period,
		"reverse", cfg.ReverseMode,
		"mode", cfg.Mode.ToString(),
		"bitrate", cfg.Bitrate,
	)

	out, err := cmd.Output()

	if ctx != nil && errors.Is(ctx.Err(), context.DeadlineExceeded) {
		cfg.Logger.Error("iperf3 command timed out",
			"mode", cfg.Mode.ToString(),
			"timeout", cfg.Timeout,
		)
		return result
	}

	if err != nil {
		stderrOutput := stderr.String()
		if stderrOutput != "" {
			cfg.Logger.Error("Failed to run iperf3",
				"mode", cfg.Mode.ToString(),
				"err", err,
				"stderr", stderrOutput,
			)
		} else {
			cfg.Logger.Error("Failed to run iperf3",
				"mode", cfg.Mode.ToString(),
				"err", err,
				"stdout", string(out),
			)
		}

		return result
	}

	// Parse the JSON output
	var raw rawResult
	if err := json.Unmarshal(out, &raw); err != nil {
		cfg.Logger.Error("Failed to parse iperf3 result",
			"mode", cfg.Mode.ToString(),
			"err", err,
		)

		return result
	}

	// Set UDPMode based on user configuration
	result.Success = true

	// Handle different metrics based on the protocol mode
	if cfg.Mode == TCP {
		// TCP Mode - use TCP-specific JSON fields
		result.SentSeconds = raw.End.SumSent.Seconds
		result.SentBytes = raw.End.SumSent.Bytes
		result.SentBitsPerSecond = raw.End.SumSent.BitsPerSecond
		result.ReceivedSeconds = raw.End.SumReceived.Seconds
		result.ReceivedBytes = raw.End.SumReceived.Bytes
		result.ReceivedBitsPerSecond = raw.End.SumReceived.BitsPerSecond
		result.Retransmits = raw.End.SumSent.Retransmits
		if len(raw.End.Streams) > 0 {
			result.MinRtt = raw.End.Streams[0].TCPSender.MinRtt
			result.MaxRtt = raw.End.Streams[0].TCPSender.MaxRtt
			result.MeanRtt = raw.End.Streams[0].TCPSender.MeanRtt
		} else {
			cfg.Logger.Warn("TCP mode: no streams found in iperf3 result")
		}
	} else {
		// UDP Mode - use UDP-specific JSON fields from streams[0].udp and sum
		// Add boundary check before accessing Streams[0]
		if len(raw.End.Streams) > 0 {
			// Common metrics using sender (streams[0].udp) data
			result.SentSeconds = raw.End.Streams[0].UDP.Seconds
			result.SentBytes = raw.End.Streams[0].UDP.Bytes
			result.SentBitsPerSecond = raw.End.Streams[0].UDP.BitsPerSecond

			// UDP-specific metrics from streams[0].udp
			result.SentPackets = raw.End.Streams[0].UDP.Packets
			result.SentJitter = raw.End.Streams[0].UDP.JitterMs
			result.SentLostPackets = raw.End.Streams[0].UDP.LostPackets
			result.SentLostPercent = raw.End.Streams[0].UDP.LostPercent
		} else {
			cfg.Logger.Warn("UDP mode: no streams found in iperf3 result")
		}

		// Common metrics using receiver (end.sum) data
		// Some versions of iperf3 might not include complete sum data for UDP
		// Access these fields safely to avoid potential issues
		result.ReceivedSeconds = raw.End.Sum.Seconds
		result.ReceivedBytes = raw.End.Sum.Bytes
		result.ReceivedBitsPerSecond = raw.End.Sum.BitsPerSecond

		// UDP-specific metrics from end.sum
		result.ReceivedPackets = raw.End.Sum.Packets
		result.ReceivedJitter = raw.End.Sum.JitterMs
		result.ReceivedLostPackets = raw.End.Sum.LostPackets
		result.ReceivedLostPercent = raw.End.Sum.LostPercent

		// Check for invalid/missing receiver metrics and log a warning
		// This can happen with some versions of iperf3
		if result.ReceivedBitsPerSecond <= 0 && result.ReceivedBytes <= 0 {
			cfg.Logger.Warn("UDP mode: missing or invalid receiver metrics in iperf3 result",
				"received_bits_per_second", result.ReceivedBitsPerSecond,
				"received_bytes", result.ReceivedBytes)
		}
	}

	// Enhanced logging with protocol-specific metrics
	if cfg.Mode == TCP {
		cfg.Logger.Debug("iperf3 TCP test completed successfully",
			"target", cfg.Target,
			"sent_bps", result.SentBitsPerSecond,
			"received_bps", result.ReceivedBitsPerSecond,
			"retransmits", result.Retransmits,
			"mean_rtt", result.MeanRtt,
		)

	} else {
		cfg.Logger.Debug("iperf3 UDP test completed successfully",
			"target", cfg.Target,
			"sent_bps", result.SentBitsPerSecond,
			"received_bps", result.ReceivedBitsPerSecond,
			"sent_jitter", result.SentJitter,
			"received_jitter", result.ReceivedJitter,
			"sent_lost_percent", result.SentLostPercent,
			"received_lost_percent", result.ReceivedLostPercent,
		)
	}

	return result
}

// CheckIperf3Exists verifies that the iperf3 command exists and is executable.
func CheckIperf3Exists() error {
	_, err := lookPath(GetIperfCmd())

	return err
}
