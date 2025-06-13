// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var version = "0.1.0-alpha"

var rootCmd = &cobra.Command{
	Use:   "gitmind",
	Short: "Git as a semantic knowledge graph",
	Long: `GitMind transforms Git into a substrate for distributed semantic memory.
It enables time-aware semantic linking of files and ideas with zero external databases.`,
	Version: version,
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func init() {
	rootCmd.AddCommand(initCmd)
}