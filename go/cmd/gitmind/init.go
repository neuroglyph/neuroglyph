// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

package main

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/spf13/cobra"
)

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initialize GitMind in the current repository",
	Long:  `Creates the .gitmind/links directory structure for storing semantic links.`,
	RunE:  runInit,
}

func runInit(cmd *cobra.Command, args []string) error {
	// Check if we're in a git repository
	if _, err := os.Stat(".git"); os.IsNotExist(err) {
		return fmt.Errorf("not in a git repository (no .git directory found)")
	}

	// Create .gitmind/links directory
	linksDir := filepath.Join(".gitmind", "links")
	if err := os.MkdirAll(linksDir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	// Create .gitkeep to ensure the directory is tracked
	gitkeepPath := filepath.Join(linksDir, ".gitkeep")
	if err := os.WriteFile(gitkeepPath, []byte(""), 0644); err != nil {
		return fmt.Errorf("failed to create .gitkeep: %w", err)
	}

	fmt.Println("Initialized GitMind in", linksDir)
	return nil
}