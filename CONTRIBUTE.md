1. _fork_ the repository https://github.com/Advanced-Programming-2023/Robotic-Lib, this will create a local repository to your account, the fork will automatically keep a link to the original upstream repository so that you can pull future changes made in the original repository
2. _clone_ your local repository to your local system so that you can start developing your own change to the repo
3. _open_ the repository in your favourite IDE and from the terminal create a new branch with a name that describe the changes you want to make `git branch <name of new branch>`
4. _checkout_ to the new branch `git checkout <name of new branch>`
5. to keep the repository in synch with the main one
	1. link your repository to the original`git remote add upstream <link to original repo>`
	2. before making any changes run the following commands:
		1. get the changes from the original repo `git fetch upstream`
		2. merge the changes on top of your local changes `git rebase upstream/master`
6. _make_ all the changes you need to do
7. _run_ `cargo test` and fix the tests your changes are breaking 
8. _run_ `cargo fmt` to format the code
9. _add_ the changes `git add .`
10. _commit_ the changes `git commit -m "good describing text"`
11. _push_ the changes `git push origin <name of new branch>`
12. return to your GitHub account on the forked repository, you should see a button "compare & pull request" you can press it
13. write a small comprehensive description about your changes and press the button "create pull request"
15. now you just need to wait approval, it is possible the maintainers will give you some feedback to improve the changes, so in order to get the pull request approved and merged you will need to follow this feedback, now every commit made in that branch will be listed in the pull request
16. after the pull request is being merged you can safely delete the branch
