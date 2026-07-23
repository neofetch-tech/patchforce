import customtkinter as ctk
from tkinter import filedialog, messagebox
try:
    import patchforge_core # type: ignore # noqa: F401
except Exception:

    import subprocess

    class _FallbackCore:
        @staticmethod
        def run_git_command(repo, args):
            cmd = ["git"] + args
            try:
                proc = subprocess.run(cmd, cwd=repo, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
                return proc.stdout
            except Exception as e:
                return f"Error running git: {e}"

    patchforge_core = _FallbackCore()


class PatchForgeGUI(ctk.CTk):
    def __init__(self):
        super().__init__()
        ctk.set_appearance_mode("Dark")
        ctk.set_default_color_theme("blue")
        self.configure(fg_color="#152041")

        self.title("PatchForge - Git GUI (Powered by Rust)")
        self.geometry("860x850")
        self.resizable(False, False)

        self.repo_path = ctk.StringVar()
        self.files_to_add = ctk.StringVar()
        self.commit_message = ctk.StringVar()
        self.branch_name = ctk.StringVar()
        self.log_count = ctk.StringVar(value="10")
        self.branch_action = ctk.StringVar(value="List branches")

        self._create_widgets()

    def _create_widgets(self):
        # 1. Repo Path
        ctk.CTkLabel(self, text="Repository path:").grid(row=0, column=0, padx=10, pady=(15, 5), sticky="w")
        ctk.CTkEntry(self, textvariable=self.repo_path, width=520).grid(row=1, column=0, padx=10, pady=5, columnspan=2, sticky="we")
        ctk.CTkButton(self, text="Browse", command=self._browse_repo).grid(row=1, column=2, padx=10, pady=5)

        # 2. Files to Add
        ctk.CTkLabel(self, text="Files to add (. for all):").grid(row=2, column=0, padx=10, pady=(10, 5), sticky="w")
        ctk.CTkEntry(self, textvariable=self.files_to_add, width=520).grid(row=3, column=0, padx=10, pady=5, columnspan=3, sticky="we")

        # 3. Commit Message
        ctk.CTkLabel(self, text="Commit message:").grid(row=4, column=0, padx=10, pady=(10, 5), sticky="w")
        ctk.CTkEntry(self, textvariable=self.commit_message, width=520).grid(row=5, column=0, padx=10, pady=5, columnspan=3, sticky="we")

        # 4. Branch Action
        ctk.CTkLabel(self, text="Branch action:").grid(row=6, column=0, padx=10, pady=(10, 5), sticky="w")
        ctk.CTkOptionMenu(
            self,
            values=["List branches", "Create branch", "Delete branch", "Switch branch"],
            variable=self.branch_action,
            width=200,
        ).grid(row=6, column=1, padx=10, pady=(10, 5), sticky="w")

        ctk.CTkLabel(self, text="Branch name:").grid(row=7, column=0, padx=10, pady=(10, 5), sticky="w")
        ctk.CTkEntry(self, textvariable=self.branch_name, width=520).grid(row=7, column=1, padx=10, pady=5, columnspan=2, sticky="we")

        # 5. Log Count
        ctk.CTkLabel(self, text="Log count:").grid(row=8, column=0, padx=10, pady=(10, 5), sticky="w")
        ctk.CTkEntry(self, textvariable=self.log_count, width=80).grid(row=8, column=1, padx=10, pady=5, sticky="w")

        # Buttons
        ctk.CTkButton(self, text="Status", width=120, command=self._status, fg_color="white", text_color="black").grid(row=9, column=0, padx=10, pady=12)
        ctk.CTkButton(self, text="Add", width=120, command=self._add_files, fg_color="white", text_color="black").grid(row=9, column=1, padx=10, pady=12)
        ctk.CTkButton(self, text="Commit", width=120, command=self._commit, fg_color="white", text_color="black").grid(row=9, column=2, padx=10, pady=12)

        ctk.CTkButton(self, text="Push", width=120, command=self._push, fg_color="white", text_color="black").grid(row=10, column=0, padx=10, pady=5)
        ctk.CTkButton(self, text="Pull", width=120, command=self._pull, fg_color="white", text_color="black").grid(row=10, column=1, padx=10, pady=5)
        ctk.CTkButton(self, text="Log", width=120, command=self._log, fg_color="white", text_color="black").grid(row=10, column=2, padx=10, pady=5)

        ctk.CTkButton(self, text="Branch", width=120, command=self._branch, fg_color="white", text_color="black").grid(row=11, column=0, padx=10, pady=5)
        ctk.CTkButton(self, text="Clear Output", width=120, command=self._clear_output, fg_color="white", text_color="black").grid(row=11, column=1, padx=10, pady=5)
        ctk.CTkButton(self, text="Exit", width=120, command=self.quit, fg_color="white", text_color="black").grid(row=11, column=2, padx=10, pady=5)

        # Output Box
        ctk.CTkLabel(self, text="Output:").grid(row=12, column=0, padx=10, pady=(10, 5), sticky="w")
        self.output_box = ctk.CTkTextbox(self, width=680, height=170, fg_color="#0A2F5A", text_color="white")
        self.output_box.grid(row=13, column=0, columnspan=3, padx=10, pady=(0, 15), sticky="we")
        self.output_box.configure(state="disabled")

    def _browse_repo(self):
        path = filedialog.askdirectory(title="Select Git repository")
        if path:
            self.repo_path.set(path)

    def _execute_rust_git(self, args):
        repo = self.repo_path.get().strip()
        if not repo:
            messagebox.showerror("Error", "Please select a valid repository folder.")
            return

        # <--- Виклики Rust-функції напряму! --->
        result_text = patchforge_core.run_git_command(repo, args)

        command_text = f"$ git {' '.join(args)}\n"
        self._append_output(command_text + result_text + "\n")

    def _append_output(self, text):
        self.output_box.configure(state="normal")
        self.output_box.insert("end", text)
        self.output_box.see("end")
        self.output_box.configure(state="disabled")

    def _clear_output(self):
        self.output_box.configure(state="normal")
        self.output_box.delete("1.0", "end")
        self.output_box.configure(state="disabled")

    # Command Methods
    def _status(self):
        self._execute_rust_git(["status"])

    def _add_files(self):
        files = self.files_to_add.get().strip()
        if not files:
            messagebox.showerror("Error", "Enter files to add (or '.' for all).")
            return
        self._execute_rust_git(["add"] + files.split())

    def _commit(self):
        msg = self.commit_message.get().strip()
        if not msg:
            messagebox.showerror("Error", "Enter a commit message.")
            return
        self._execute_rust_git(["commit", "-m", msg])

    def _push(self):
        b = self.branch_name.get().strip()
        args = ["push"] if not b else ["push", "origin", b]
        self._execute_rust_git(args)

    def _pull(self):
        b = self.branch_name.get().strip()
        args = ["pull"] if not b else ["pull", "origin", b]
        self._execute_rust_git(args)

    def _log(self):
        count = self.log_count.get().strip() or "10"
        self._execute_rust_git(["log", "-n", count, "--oneline"])

    def _branch(self):
        action = self.branch_action.get()
        name = self.branch_name.get().strip()

        if action == "List branches":
            args = ["branch", "-a"]
        elif action == "Create branch":
            if not name:
                messagebox.showerror("Error", "Enter a branch name.")
                return
            args = ["branch", name]
        elif action == "Delete branch":
            if not name:
                messagebox.showerror("Error", "Enter a branch name.")
                return
            args = ["branch", "-d", name]
        elif action == "Switch branch":
            if not name:
                messagebox.showerror("Error", "Enter a branch name.")
                return
            args = ["checkout", name]
        else:
            return

        self._execute_rust_git(args)


if __name__ == "__main__":
    app = PatchForgeGUI()
    app.mainloop()