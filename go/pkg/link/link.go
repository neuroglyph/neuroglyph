// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

package link

import (
	"crypto/sha1"
	"fmt"
	"time"
)

// LinkType represents the type of semantic relationship
type LinkType string

const (
	CrossRef   LinkType = "CROSS_REF"
	DependsOn  LinkType = "DEPENDS_ON"
	Implements LinkType = "IMPLEMENTS"
	InspiredBy LinkType = "INSPIRED_BY"
	References LinkType = "REFERENCES"
	Tests      LinkType = "TESTS"
)

// Link represents a semantic relationship between two files
type Link struct {
	Type      LinkType  `json:"link_type"`
	Source    string    `json:"source"`
	Target    string    `json:"target"`
	Timestamp time.Time `json:"timestamp"`
}

// NewLink creates a new link with the current timestamp
func NewLink(linkType LinkType, source, target string) *Link {
	return &Link{
		Type:      linkType,
		Source:    source,
		Target:    target,
		Timestamp: time.Now(),
	}
}

// ToCanonical returns the canonical string representation of the link
func (l *Link) ToCanonical() string {
	return fmt.Sprintf("%s: %s -> %s  # ts:%d",
		l.Type, l.Source, l.Target, l.Timestamp.Unix())
}

// SHA returns the SHA-1 hash of the canonical representation
func (l *Link) SHA() string {
	h := sha1.New()
	h.Write([]byte(l.ToCanonical()))
	return fmt.Sprintf("%x", h.Sum(nil))
}

// Filename returns the filename for storing this link
func (l *Link) Filename() string {
	return l.SHA() + ".link"
}