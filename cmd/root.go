package cmd

import (
	"fmt"
	"os"

	"github.com/koenverburg/committer/helpers"
	"github.com/koenverburg/committer/questions"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "committer",
	Short: "Committer is a simple but powerfull commit message creator tool",
	Long:  `to be written`,
	Run: func(cmd *cobra.Command, args []string) {
		commit := questions.RunForm()
		helpers.Commit(commit.Msg, commit.Description)
	},
}

// var autoCmd = &cobra.Command{
// 	Use:   "auto",
// 	Short: "Print the version number of Hugo",
// 	Long:  `All software has versions. This is Hugo's`,
// 	Run: func(cmd *cobra.Command, args []string) {
// 		fmt.Println("Hugo Static Site Generator v0.9 -- HEAD")
// 	},
// }

// func init() {
// 	rootCmd.AddCommand(autoCmd)
// }

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
