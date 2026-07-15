Output assets/demo.gif

Require stab

Set Shell "bash"
Set FontSize 24
Set Width 900
Set Height 500
Set TypingSpeed 50ms
Set Theme "GitHub Dark"

# 0. Help output — context setting
Type "stab --help" Sleep 500ms Enter
Sleep 3s

# Clear screen for clean transition
Type "clear" Sleep 200ms Enter
Sleep 500ms

# 1. Show data with bat
Type "bat data/test.csv" Sleep 500ms Enter
Sleep 3s

# Clear screen for clean transition
Type "clear" Sleep 200ms Enter
Sleep 500ms

# 2. Read CSV — the main show
Type "stab data read data/test.csv" Sleep 500ms Enter
Sleep 4s

# Clear screen
Type "clear" Sleep 200ms Enter
Sleep 500ms

# 3. Describe — schema inspection
Type "stab data describe data/test.csv" Sleep 500ms Enter
Sleep 4s

# Clear screen
Type "clear" Sleep 200ms Enter
Sleep 500ms

# 4. JSON output — pipeline integration
Type "stab data read data/test.csv --output json" Sleep 500ms Enter
Sleep 3s