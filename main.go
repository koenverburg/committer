package main

import "github.com/koenverburg/committer/internal"

func main() {
	commitMessage := internal.RunForm()
	internal.Commit(commitMessage.Msg, commitMessage.Description)
}
