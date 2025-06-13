# Gitoxide API Issues

## Description

Ran into some issues trying to stage a deleted file during an edge case test.

### Details from Claude:
```
Let me clarify the gitoxide API issues:

Gitoxide API Issues Encountered

Looking back at the changes, I only had to change 3 specific operations to use CLI instead of gitoxide:

1. `add()` - The `index_or_empty()` method returns an `Rc<FileSnapshot<File>>` which doesn't have mutable methods
2. `commit()` - The `commit_as()` method signature changed (takes 6 args not 5, different parent handling)
3. `remove()` - The index type doesn't have a remove() method

The `is_repository()` method still uses gitoxide and works fine!

⏺ Read(cli/src/git.rs)
⎿  Read 40 lines (ctrl+r to expand)

⏺ The issues seem to be:
1. We're using `gix 0.64` but the API may have changed
2. The index manipulation API is more complex than expected
3. Some methods we expected don't exist or have different signatures

We could file a GitHub issue, but it might be worth first:
1. Checking if we're using the right features in Cargo.toml
2. Looking at gix examples to see the correct API usage
3. Possibly updating to a newer version

For now, the CLI fallback works perfectly fine and all tests pass!
```

### Details from James:

When he was trying to get these tests to pass, he hit a loop of failures. I interjected and recommended that he just try doing `git rm` to see if that would be a valid workaround. Seemed like it did work, so he rolled with that. 

I would like to avoid shelling out to CLI git if possible.

#### Possibly I've been confused because of bad git habits

```
You’re not going crazy — but you are hitting a subtle distinction in Git behavior that’s always been there, but might feel different depending on context or tooling.

TL;DR:
	•	git add <file> stages modifications or new files — but not deletions.
	•	git add -u or git add . will stage deletions.
	•	git rm <file> stages the deletion and removes it from your working directory.

So why does it feel different now?

Possibilities:
	1.	Your workflow/tool changed:
	•	If you used to use git add . or git add -u, it would catch deletions.
	•	If you switched to git add <file>, it won’t — and never did.
	2.	You’re using a GUI or IDE that auto-staged deletions before, and now you’re using CLI more — or vice versa.
	3.	You might’ve used git add -A, which stages everything (adds, modifies, deletes) and it felt like git add <file> because it worked the way you expected.

⸻

The Commands You Want

Action	Git Command
Stage a specific file	git add foo.txt
Stage a deletion	git rm foo.txt or git add -u
Stage all changes (safe)	git add -u
Stage everything (danger)	git add -A
See what’s staged	git status


⸻

Why this design?

Git tracks the index (what will be committed), not the working tree directly. git add <file> only stages content for that path — but if the file is gone, there’s no content to stage. Git assumes you might want to recover it unless you say “yes, I deleted it” (via git rm, add -u, etc).

⸻

Pro Tip (2025 edition):

Want deletions + modifications + new files all at once? Use:

git add -A

Or for a safer middle-ground (no new untracked files):

git add -u


⸻

No, Git itself hasn’t changed this behavior recently — it’s just deceptively subtle. The weirdness is us changing workflows or tools and expecting Git to be psychic.

Let me know your current setup and I’ll tell you the exact minimal command to get your workflow back to how it felt.
```

I'll have to have Claude double check _his_ git fu...

## Recommendation

1. Make sure we're using the latest gix version
2. Check their github issues, see if anyone else has noticed this problem
3. Are we sure this isn't because of something wacky we did? Like test doubles preventing stuff? Does the docker container that runs the tests have a bad version of git? Git fu wack?