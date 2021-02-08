# Controllers

## Find and Replace (Renaming Event variants)
This shouldn't need to be done too often, but so I don't forget:

```sh
find . \( -type d -name .git -prune \) -o -type f -print0 | xargs -0 sed -i 's/JoyZ/JoyZ/g'
```
