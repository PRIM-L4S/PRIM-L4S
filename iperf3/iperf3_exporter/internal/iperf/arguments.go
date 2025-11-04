package iperf

import "strconv"

func (cfg *Config) generateIperfArgs() []string {
	iperfArgs := []string{
		"-J",
		"-t", strconv.FormatFloat(cfg.Period.Seconds(), 'f', 0, 64),
		"-c", cfg.Target,
		"-p", strconv.Itoa(cfg.Port),
	}

	if cfg.ReverseMode {
		iperfArgs = append(iperfArgs, "-R")
	}

	if cfg.Mode == UDP {
		iperfArgs = append(iperfArgs, "-u")
	}

	// Apply bitrate:
	// - For UDP: use specified bitrate or default to "1M" if none specified (iperf3 defaults to 1Mbps for UDP)
	// - For TCP: only apply if explicitly specified (iperf3 defaults to unlimited for TCP)
	if cfg.Mode == UDP {
		if cfg.Bitrate != "" {
			iperfArgs = append(iperfArgs, "-b", cfg.Bitrate)
		} else {
			// Default to 1Mbps for UDP if not specified
			iperfArgs = append(iperfArgs, "-b", "1M")
			cfg.Logger.Debug("Using default 1Mbps bitrate for UDP mode")
		}
	} else if cfg.Bitrate != "" {
		// Only apply bitrate for TCP if explicitly specified
		iperfArgs = append(iperfArgs, "-b", cfg.Bitrate)
	}

	// Only useful for development on Darwin
	// iperfArgs = append(iperfArgs, "-w", "17336")

	return iperfArgs
}
