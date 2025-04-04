# Creating a Custom ASCII Logo for SCU

## Color Format

To add colors to your ASCII logo, use the following format:

```
${color_number}
```

When the CLI runs, this will be transformed into the ANSI color code:

```
\x1b[38;5;{color_number}m
```

This format allows you to add 256 different colors to your ASCII text logo.

## Where to Find Color Codes

https://hexdocs.pm/color_palette/ansi_color_codes.html

## Creating Your Logo

1. Design your ASCII art logo using characters like `-`, `|`, `/`, `\`, `_`, etc.
2. Add color codes using the `${number}` format before each section you want to colorize

## Example Logo

```
        ${8}#####
       #######
       ##${7}^${8}#${7}^${8}##
       #${3}#####${8}#
     ##${7}##${3}###${7}##${8}##
    #${7}##########${8}##
   #${7}############${8}##
   #${7}############${8}###
  ${3}##${8}#${7}###########${8}##${3}#
######${8}#${7}#######${8}#${3}######
#######${8}#${7}#####${8}#${3}#######
  ${3}#####${8}#######${3}#####
```

Here's how the logo looks in the terminal:

<img src="./images/logo_sample.png">

## Implementation Notes

- The color is NOT reset to the default at the end of each line
- Use color code 0 to reset to the default terminal color: `${0}`
- Make sure your logo fits within typical terminal width constraints
- Test your logo in different terminal types to ensure compatibility

## Tips for Great ASCII Logos

- Keep it simple but recognizable
- Use contrasting colors for better visibility
- Consider using a maximum of 3-4 colors for visual clarity
- Maintain consistent style throughout the logo
