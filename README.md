# Hook Escape Calculator
## Description
A calculator for the odds of escape from the hook in the video game _Dead By Daylight_.

The full odds of escaping from the hook in _Dead by Daylight_ is not a trivial mental calculation. It's a calculation which suffers from a particularly common cognitive bias, and attempting to diligently perform the true calculation may not be convenient while a person is actively playing a match. This Hook Escape Calculator correctly calculates the probability of success with great convenience.

## Installation
### Direct Download
Future versions of this application may be directly downloadable from the "Releases" tab on GitHub.

### Build from Source
As with many Rust applications, the creation of applications from source is trivial after a correct installation of Rust.

1. Ensure that [you have correctly installed Rust](https://www.rust-lang.org/tools/install).
2. Install the repository
    - Build from github URL with `cargo install --git https://github.com/MartinOlsenDev/hook_escape_calculator --bin Hook-Escape-Calculator` (Recommended)
    - or; Download and Build at a Local Directory
      1. [Clone this repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository)
      2. `cargo install --path hook_escape_calculator --bin Hook-Escape-Calculator`

The executable will now be available at `CARGO_HOME>bin>Hook-Escape-Calculator` which is by default at `\%USER%\.cargo\bin` in windows and at `~/.cargo/bin` in unix-like operating systems.

Please understand that `.cargo` is a hidden folder, so ensure that you have the appropriate visibility settings when navigating to the binary. Users on any operating system should be able to invoke the binary with the shell command `$ Hook-Escape-Calculator` or by linking an icon to the executable by the method appropriate to the operating system.

## Features
Hook Escape Calculator allows users to view a grid of survivors in a match of Dead by Daylight. Users can select the non-trivial luck perks and offerings which exist, and they may also update the game state for values that may effect luck. The app will automatically update all survivors' chances of escaping after one unhook attempt and all of their unhook attempts.

The application has a small footprint both while running ($<30$ MiB) and while stored ($<10$ MiB). This enables easy downloading and low resource usage while running.

## Safety
See the `LICENSE.txt` for formal legal liability. While no guarantees are provided, this app performs absolutely no memory scanning or injection, so it should not be deemed dangerous by a typical anti-cheat engine. A naive anti-cheat engine may see the keyword "Hook" and "Calculator" in the process name and activate, but that would be very poorly designed software.

## Explaination of Operation
This calculator uses a custom library backend to calculate probabilities, and then it allows the user to observe and alter those probabilities with the iced GUI library, which is a message-passing GUI library.

### How Luck and Unhook Attempts are Calculated
Survivors in Dead by Daylight have a luck stat which is a percentage in the range [$0%$, $100%$], which will be henceforth labeled $l$. It also gives survivors some number of attempts to unhook themselves, which will be henceforth labeled $u$. The chance of escaping over all unhook attempts is $1 - (1 - l)^u$.

This calculation runs contrary to human cognitive bias, which may anticipate that the equation is $l*u$. This is because the correct equation requires the complement of a complement to correctly evaluate, which many people do not find intuitive.

Furthermore, both luck and the unhook attempt chances may vary from survivor to survivor. Various survivor equippables may contribute to either the global luck or to personal luck. Global luck benefits all survivors while personal luck only benefits a survivor who is associated with it. Luck, $l$, may be described as the sum of personal luck and global luck.

Not only do these factors change as different equippable objects are chosen, but these factors may change as the game progresses based on certain in-game events.

A survivor may have up to four perks and up to one offering, among other (irrelevant) equippables. There are 2 equippable (non-trivial) luck-affecting perks and 6 equippable (non-trivial) luck-affecting offerings.

#### Trivial Luck Equippables
There are two trivial luck equippables that will not be described at length. Deliverance and Wicked both conditionally lock a survivors luck precisely at $100%$. This is a trivial calculation, as certain success in one attempt is trivially at least one success across many attempts. Trivial luck equippables do not appear within this application.

#### Non-Trivial Luck Equippables
##### Offerings
As stated, there are 6 luck offerings. There are two luck offerings each that contribute $1%$, $2%$, and $3%$. For these pairs of luck offerings on each percent, there is one that contributes that luck personally and one that contributes it globally. All personal offerings reference chalk and all global offerings reference salt.

##### Perks
###### Slippery Meat
Slippery Meat always increases the maximum number of self unhook attempts by 3 and also raises personal luck by 2%, 3%, or 4% based on tier.

###### Up the Ante
Up the Ante contributes global luck based on an unintuitive formula. Up the Ante has an associated coefficient that comes with its tier, $x$, that is used for calculation. Up the Ante's impact can be stated as: let $n$ be the number of living survivors except the user of this instance of Up the Ante. The instance of Up the Ante contributes $n * x$ global luck while its user alive.

#### The Luck Record Family of Types
The Luck Record family of types is the logical structure that powers the luck engine. There are 3 Luck Record types: Loadout, Personal, and Team. There are also converters from Loadout to Personal and from Personal to Team.

A Luck Record has knowledge of abstract luck contributions which have occurred, but not always what contributed them. For example, a Salt Statuette would simply recorded in a luck record as a global $2%$ contribution of luck.

Every equippable in the game can be simply converted into a Loadout Luck record. Because a Loadout Luck record is blind to its sources, we can combine them arbitrarily to create all the luck-relevant information that is known on the loadout screen.

Then, because some information may be required that can only be known with the context of a whole player in game, a converter is created. The converter converts the combined Loadout Luck record into a Player Luck record.

Next, some luck information may require the context of the entire game state to be decidable (ex Up the Ante's final global luck contribution). Accordingly, a converter between Player and Team is created, which creates a Team Luck record.

A team luck record only contains the running total of global luck and a list of pairs where the left item represents a survivor's personal luck and the right item represents their maximum unhook attempt count modifier.

From here, we can simply apply our old friend $1 - (1 - l)^u$.

#### The Constants File and Its Benefits
The constants file at `src/lib/constants.rs` simplifies most of the probable future game-changes. Because certain numbers are defined only once project wide (such as percents given by certain offerings), then only the constants file needs to be modified to represent those changes.

### Bug Reporting
Generally, the best practice for bug reporting is to identify a case where the anticipated result and the actual result differ. Below are two anticipated possible bugs.

#### Probability Errors
Any error in the display of probability is considered a critical error. Please take a screenshot of the entire application window and write out how the actual value differs from the anticipated value.

#### Display Errors
If the application is drawing to the screen incorrectly, please take a picture or video showcasing the improper draw and write out how the draw differs from what should be present. Please note that if your bug is due to an unusual display resolution or size, the fix may be low-priority.

### Contributing
hook_escape_calculator welcomes contributors. The following areas especially could use some love.

- Assets

        Currently, there are no visual assets for this app. Instead of having an icon for the perks and offerings, the app only displays text. Creators who can create recognizible assets representing luck items would be appreciated.

        Previous demonstrable experience creating Dead by Daylight perk packs or general asset creation preferred. Artists should consider the implications of the `LICENSE.txt` on their work. If you are interested in this, please express that interest in issue tracker #17 prior to creating any assets.

- User Interface

        The user interface of this application is servicable, but could be beautified aesthetically and simplified practically. Users willing to work with iced to create a more streamlined design in both respects would be welcome.

### Consuming as a Lib
The library probability engine and user interface for this app are not tightly coupled. A new crate could choose to consume this app solely for its library and create its own interface. User developers are invited to do so in accordance with the `LICENSE.txt`.