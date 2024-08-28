package helpers

import (
	"testing"
)

func TestCreateMessage(t *testing.T) {
	tables := []struct {
		ticket   string
		change   string
		subject  string
		message  string
		tags     []string
		expected string
	}{
		{"", "feat", "scope", "subject", []string{"Tag1"}, "feat(scope) subject Tag1\n"},
		{"", "feat", "", "subject", []string{"Tag1"}, "feat: subject Tag1\n"},
		{"AB-11111", "feat", "", "subject", []string{"Tag1"}, "AB-11111 feat: subject Tag1\n"},
		{"AB-11111", "feat", "scope", "subject", []string{"Tag1"}, "AB-11111 feat(scope) subject Tag1\n"},
		{"AB-11111", "feat", "scope", "subject", []string{}, "AB-11111 feat(scope) subject\n"},
		{"AB-11111", "feat", "", "subject", []string{}, "AB-11111 feat: subject\n"},
		{"", "feat", "", "subject", []string{}, "feat: subject\n"},
	}

	for _, table := range tables {
		result := CreateMessage(table.ticket, table.change, table.subject, table.message, table.tags)
		if result != table.expected {
			t.Errorf("CreateMessage of (%s, %s, %s, %s, %v) was incorrect, got: %s, want: %s.",
				table.ticket, table.change, table.subject, table.message, table.tags, result, table.expected)
		}
	}
}
