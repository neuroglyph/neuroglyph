// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

package main

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/neuroglyph/gitmind/pkg/link"
	"github.com/spf13/cobra"
)

var linkType string

var linkCmd = &cobra.Command{
	Use:   "link <source> <target>",
	Short: "Create a semantic link between two files",
	Long:  `Creates a semantic relationship between source and target files, stored in .gitmind/links/`,
	Args:  cobra.ExactArgs(2),
	RunE:  runLink,
}

func init() {
	linkCmd.Flags().StringVarP(&linkType, "type", "t", "CROSS_REF", "Type of link (CROSS_REF, DEPENDS_ON, IMPLEMENTS, INSPIRED_BY, REFERENCES, TESTS)")
	rootCmd.AddCommand(linkCmd)
}

func runLink(cmd *cobra.Command, args []string) error {
	source := args[0]
	target := args[1]

	// Validate files exist
	if _, err := os.Stat(source); os.IsNotExist(err) {
		return fmt.Errorf("source file does not exist: %s", source)
	}
	if _, err := os.Stat(target); os.IsNotExist(err) {
		return fmt.Errorf("target file does not exist: %s", target)
	}

	// Create link
	lt := link.LinkType(linkType)
	l := link.NewLink(lt, source, target)

	// Write link file
	linksDir := filepath.Join(".gitmind", "links")
	linkPath := filepath.Join(linksDir, l.Filename())
	
	content := l.ToCanonical() + "\n"
	if err := os.WriteFile(linkPath, []byte(content), 0644); err != nil {
		return fmt.Errorf("failed to write link file: %w", err)
	}

	fmt.Printf("Created link: %s -> %s (%s)\n", source, target, linkType)
	return nil
}