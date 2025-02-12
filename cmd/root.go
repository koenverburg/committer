package cmd

import (
	"fmt"
	"os"

	"github.com/koenverburg/committer/git"
	"github.com/koenverburg/committer/internal"
	"github.com/koenverburg/committer/questions"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "committer",
	Short: "Committer is a simple but powerfull commit message creator tool",
	Long:  `to be written`,
	Run: func(cmd *cobra.Command, args []string) {
		commit := questions.StartCommitForm()

		commitFlags := questions.StartCommitFlagForm()
		pushFlags := questions.StartPushForm()

		commitCmdStr := internal.FormatCommitString(commit.Msg, commit.Description, commitFlags.NoVerify)
		pushCmddStr := internal.FormatPushString(pushFlags.PushWithForce)

		err := git.Commit(commitCmdStr)
		internal.CheckIfErrorFatal(err)

		if pushFlags.Push {
			err := git.Push(pushCmddStr)
			internal.CheckIfErrorFatal(err)
		}
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
