# Git commands
A set of commands typically used in Git. Placed them here for easy acces/navigation. 🙂

---

### **1. Basic Git Workflow (Local Repo)**
| Command | Purpose |
|---------|---------|
| `git init` | Initialize a new Git repo in your project folder. |
| `git add <file>` | Stage a specific file for commit. |
| `git add .` | Stage **all** changed files for commit. |
| `git commit -m "message"` | Commit staged changes with a descriptive message. |
| `git status` | Check which files are staged/modified/untracked. |
| `git log` | View commit history. |

---

### **2. Branch Management**
| Command | Purpose |
|---------|---------|
| `git branch -M main` | Rename the default branch to `main` (modern practice). |
| `git branch` | List all local branches. |
| `git checkout -b <branch>` | Create and switch to a new branch. |

---

### **3. Syncing with Remote (GitHub/GitLab)**
| Command | Purpose |
|---------|---------|
| `git remote add origin <url>` | Link your local repo to a remote repository. |
| `git push -u origin main` | First push (sets upstream tracking for `main`). |
| `git push origin main` | Push commits to `main` (after the first push). |
| `git pull origin main` | Fetch remote changes **and** merge them locally. |
| `git fetch` | Check for remote changes **without** merging. |

---

### **4. Undoing Mistakes**
| Command | Purpose |
|---------|---------|
| `git restore <file>` | Discard unstaged changes to a file. |
| `git reset --soft HEAD~1` | Undo last commit (keep changes staged). |
| `git revert <commit-hash>` | Safely undo a pushed commit. |

---

### **Key Notes:**
- **`git push -u origin main`** is only needed **once** (the `-u` sets upstream tracking).  
- After that, **`git push`** suffices if you’re on `main`.  
- **`git fetch`** checks for remote changes but doesn’t merge them (use `git pull` to fetch + merge).  

---

### **Example Workflow:**
1. Make changes to `file.txt`.
2. Stage and commit:
   ```bash
   git add file.txt
   git commit -m "Update file.txt"
   ```
3. Push to remote:
   ```bash
   git push origin main
   ```

---

### **Some Other Important Commands**
- `git clone <url>` (download a remote repo).  
- `git merge <branch>` (merge branches).