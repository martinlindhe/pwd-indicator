# ABOUT
A tiny component to help set current path title in Powershell


# WHY
Windows Terminal currently lacks a feature to set tab titles to the current path


# HOW

Using [starship](https://starship.rs/), set up this PreCommand in your powershell profile:

```ps1
function Invoke-Starship-PreCommand {
    $Host.UI.RawUI.WindowTitle = $(pwd-indicator)
}
```



# TODO

more ways to customize output, like:

(TODO) full: C:\Users\m\dev\rs\pwd-indicator
(TODO) relative:  ~/dev/rs/pwd-indicator
(TODO) relative vs git root:  pwd-indicator/target/release   (starship prompt has this, could maybe reuse)
(DONE) current dir only: pwd-indicator

TODO: flag for using windows or linux path separators
TODO: flag for presentation mode
