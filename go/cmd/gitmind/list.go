// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/spf13/cobra"
)

var listCmd = &cobra.Command{
	Use:   "list",
	Short: "List all semantic links",
	Long:  `Lists all semantic links stored in .gitmind/links/`,
	RunE:  runList,
}

func init() {
	rootCmd.AddCommand(listCmd)
}

func runList(cmd *cobra.Command, args []string) error {
	linksDir := filepath.Join(".gitmind", "links")
	
	// Check if directory exists
	if _, err := os.Stat(linksDir); os.IsNotExist(err) {
		return fmt.Errorf("no .gitmind/links directory found (run 'gitmind init' first)")
	}

	// Read all .link files
	entries, err := os.ReadDir(linksDir)
	if err != nil {
		return fmt.Errorf("failed to read links directory: %w", err)
	}

	found := false
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".link") {
			continue
		}

		// Read and display link
		path := filepath.Join(linksDir, entry.Name())
		file, err := os.Open(path)
		if err != nil {
			continue
		}
		defer file.Close()

		scanner := bufio.NewScanner(file)
		if scanner.Scan() {
			fmt.Println(scanner.Text())
			found = true
		}
	}

	if !found {
		fmt.Println("No links found")
	}

	return nil
}