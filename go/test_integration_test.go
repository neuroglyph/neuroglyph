// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

package main_test

import (
	"os"
	"os/exec"
	"path/filepath"
	"testing"
)

func TestInitCommand(t *testing.T) {
	// Create temp directory for test
	tmpDir, err := os.MkdirTemp("", "gitmind-test-*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	// Initialize git repo in temp dir
	cmd := exec.Command("git", "init")
	cmd.Dir = tmpDir
	if err := cmd.Run(); err != nil {
		t.Fatal("Failed to init git repo:", err)
	}

	// Build gitmind binary
	buildCmd := exec.Command("go", "build", "-o", filepath.Join(tmpDir, "gitmind"), "./cmd/gitmind")
	if err := buildCmd.Run(); err != nil {
		t.Fatal("Failed to build gitmind:", err)
	}

	// Run gitmind init
	gitmindCmd := exec.Command(filepath.Join(tmpDir, "gitmind"), "init")
	gitmindCmd.Dir = tmpDir
	if err := gitmindCmd.Run(); err != nil {
		t.Fatal("gitmind init failed:", err)
	}

	// Check that .gitmind/links directory was created
	linksDir := filepath.Join(tmpDir, ".gitmind", "links")
	if _, err := os.Stat(linksDir); os.IsNotExist(err) {
		t.Error(".gitmind/links directory was not created")
	}
}