Output demo.gif

# Settings
Set FontSize 15
Set Theme { "black": "#51576d", "red": "#e78284", "green": "#a6d189", "yellow": "#e5c890", "blue": "#8caaee", "purple": "#f4b8e4", "cyan": "#81c8be", "white": "#b5bfe2", "brightBlack": "#626880", "brightRed": "#e78284", "brightGreen": "#a6d189", "brightYellow": "#e5c890", "brightBlue": "#8caaee", "brightPurple": "#f4b8e4", "brightCyan": "#81c8be", "brightWhite": "#a5adce", "background": "#303446", "foreground": "#c6d0f5", "selectionBackground": "#626880", "cursorColor": "#f2d5cf" }
Set Framerate 60
Set PlaybackSpeed 2.0 # change this later
Set Padding 0
Set TypingSpeed 100ms

# Actions

# --- Enter the App ---
Type "tood"
Enter

# --- Todo Navigation ---
Type "jjjkj"

# --- Toggle Completion ---
Space
Sleep 200ms
Space
Sleep 200ms
Space
Sleep 200ms
Space
Sleep 200ms

# --- Add Todo ---

# Add title
Type "a"
Type@50ms "Make a demo for tood"
Sleep 300ms

# Add description
Ctrl+E
Type "i"
Type "Show off in the demo that you can edit your todo descriptions in $EDITOR"
Sleep 300ms

# Exit
Ctrl+H
Type ":wq"
Enter
Sleep 500ms

# Add due date
Ctrl+D
Type "jhhlkLLLkk"
Tab
Sleep 300ms
Type "jjljj"

Enter
Enter

Sleep 500ms

# --- Edit Todo ---
Type "e"
Backspace 4
Type@50ms "this cool application"
Sleep 300ms

# Add description
Ctrl+E
Type "o"
Sleep 200ms
Type "Now I'm editing the description"
Sleep 300ms

# Exit editor
Ctrl+H
Type ":wq"
Enter
Sleep 500ms
Enter

# Mark complete
Sleep 300ms
Space
Sleep 300ms


# Add recurring
Type "a"
Type@50ms "Recurring todo just because"
Sleep 300ms
Ctrl+R
Enter
Sleep 500ms

# --- Skimmer ---
Type "f"
Sleep 300ms
Type "nent"
Sleep 500ms
Tab
Sleep 100ms
Enter
Sleep 500ms
Type "f"
Sleep 300ms
Type "just because"
Sleep 500ms
Tab
Sleep 100ms
Enter

# --- Move mode ---
Type@200ms "mkkkjjj"
Enter

# --- Delete Todos ---
Type@200ms "dd"

Sleep 500ms
Type "q"
