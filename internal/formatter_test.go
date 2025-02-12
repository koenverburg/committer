package internal

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFormatCommitMessage(t *testing.T) {
	tables := []struct {
		ticket   string
		change   string
		subject  string
		message  string
		tags     []string
		expected string
	}{
		{"", "feat", "scope", "subject", []string{"Tag1"}, "feat(scope) subject Tag1"},
		{"", "feat", "", "subject", []string{"Tag1"}, "feat: subject Tag1"},
		{"AB-11111", "feat", "", "subject", []string{"Tag1"}, "AB-11111 feat: subject Tag1"},
		{"AB-11111", "feat", "scope", "subject", []string{"Tag1"}, "AB-11111 feat(scope) subject Tag1"},
		{"AB-11111", "feat", "scope", "subject", []string{}, "AB-11111 feat(scope) subject"},
		{"AB-11111", "feat", "", "subject", []string{}, "AB-11111 feat: subject"},
		{"", "feat", "", "subject", []string{}, "feat: subject"},
	}

	for _, table := range tables {
		result := FormatCommitMessage(table.ticket, table.change, table.subject, table.message, table.tags)
		if result != table.expected {
			t.Errorf("CreateMessage of (%s, %s, %s, %s, %v) was incorrect, got: %s, want: %s.",
				table.ticket, table.change, table.subject, table.message, table.tags, result, table.expected)
		}
	}
}

func TestFormatCommitString(t *testing.T) {
	assert := assert.New(t)

	all := FormatCommitString("message", "description", false)
	assert.Equal("commit, -m message, -m description, --quiet", all, "Normal commit command")

	noDescription := FormatCommitString("message", "", false)
	assert.Equal("commit, -m message, --quiet", noDescription, "Normal commit command")

	noVerify := FormatCommitString("message", "description", true)
	assert.Equal("commit, -m message, -m description, --quiet, --no-verify", noVerify, "no-verify command")
}

func TestFormatPushString(t *testing.T) {
	assert := assert.New(t)

	normal := FormatPushString(false)
	assert.Equal("push, --quiet", normal, "Normal push command")

	force := FormatPushString(true)
	assert.Equal("push, --quiet, --force", force, "Force push command")
}
