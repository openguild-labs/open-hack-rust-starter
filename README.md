# OpenHack Workshop - Rust Starter

![image](https://github.com/openguild-labs/open-hack-rust-starter/assets/56880684/3f07ca81-d5fa-4654-ad3d-74ed4ab05070)

Repositoriy is made by the **OpenGuild Labs** to introduce OpenHack workshop participants about Rust programming language and help the participants to get familiar with the language.

## Participant Registration

Add your information to the below list to officially participate in the workshop challenge (This is the first mission of the whole workshop)

| Emoji | Name            | Github Username                                       | Occupations              |
| ----- | --------------- | ----------------------------------------------------- | ------------------------ |
| 🦀    | Phu Sy          | [phusy2001](https://github.com/phusy2001)             | Developer                |
| 🦀    | Anh Pham        | [FucktheKingcode](https://github.com/FucktheKingcode) | Dev \*                   |
| 🦀    | Tin Chung       | [chungquantin](https://github.com/chungquantin)       | DevRel Lead Polkadot SEA |
| 🦀    | Dat Nguyen      | [hnimtadd](https://github.com/hnimtadd)               | Software engineer        |
| 🦀    | Phap Luong      | [phapdev](https://github.com/phapdev)                 | Student                  |
| 🦀    | Lucas           | [Lucas](https://github.com/nvtuanqti212)              | Backend Engineer         |
| 🦀    | Nguyen Ngoc Phu | [ngyngcphu](https://github.com/ngyngcphu)             | Student                  |
| 🐷    | Huy Do          | [Huy-DNA](https://github.com/Huy-DNA)                 | Software engineer        |
| 🦀    | Hoang Lam       | [lamdanghoang](https://github.com/lamdanghoang)       | Student                  |
| 🦀    | Tuong Nguyen    | [zrus](https://github.com/zrus)                       | Developer                |
| 🦀    | Khiem Cong    | [congnghiakhiem](https://github.com/congnghiakhiem)                       | IT Researcher                |
| 🦀    | Anh Tuan        | [chauanhtuan185](https://github.com/chauanhtuan185) | Software Engineer          |

## Learn more about OpenGuild

- **About us:** [Learn more about us](https://openguild.wtf/about)
- **Website:** [OpenGuild Website](https://openguild.wtf/)
- **Github:** [OpenGuild Labs](https://github.com/openguild-labs)
- **Discord**: [Openguild Discord Channel](https://discord.gg/bcjMzxqtD7)

## Goals of the workshop

1. Get familiar with Git & Github via open-source contribution
   - Install `git` on your local device
   - Git `fork` and `clone` command
   - `commit` and `push` code from your local device to Github
   - Create a `Pull Request` and merge with this repository
2. Install and setup Rust on your local device
3. Hands-on experience with a few Rust exercises
4. Collaborate with other participants to work on the **"One Billion Row" (1brc)** challenge.
5. Compete your team solution with the another and receive a bounty for being the best team solution.

---

## Mission 1: Get familiar with Git & Github via open-source contribution

- Step 1: Install Git & Github Desktop (optional) on your local device
- Step 2: Fork this repository by click the `Fork button` on Github

![image](https://github.com/openguild-labs/open-hack-rust-starter/assets/56880684/7fa2f01a-b523-4208-92db-d8af7a274d98)

<img width="683" alt="Screenshot 2024-04-19 at 16 20 00" src="https://github.com/openguild-labs/open-hack-rust-starter/assets/56880684/138d1d07-5c0f-4fc3-b73d-8eafae3c692d">

- Step 3: `Clone` the forked repository to your local device using the below command

```
git clone https://github.com/[name-of-your-account]/open-hack-rust-starter.git
```

Replace the `[name-of-your-account]` with your Github username. For example, if my username is `chungquantin`, I would do the below command to clone the repository to my local device.

```
git clone https://github.com/chungquantin/open-hack-rust-starter.git
```

- Step 4: Edit the `README.md` file to register for official participation

Go to **Participant Registration** section and register to be the workshop participants. Add the below to the list, replace any placeholder with your personal information.

```
| 🦀    | Your Name | Your Github username | Your current occupation |
```

- Step 5: `Commit` your code and push to the forked Github repository

```
git add .
git commit -m "Register for OpenHack workshop"


- Step 6: Create a `Pull Request` to merge your changes to this repository and name your PR as `Your name | Register for OpenHack workshop`

<img width="1166" alt="Screenshot 2024-04-19 at 16 23 45" src="https://github.com/openguild-labs/open-hack-rust-starter/assets/56880684/7554ca7d-da68-4a23-893a-4f2c11a78d37">

---

## Mission 2: Install and setup Rust on your local device

The first step is to install Rust. We’ll download Rust through `rustup`, a
command line tool for managing Rust versions and associated tools. You’ll need
an internet connection for the download.

> Note: If you prefer not to use `rustup` for some reason, please see the
> [Other Rust Installation Methods page][otherinstall] for more options.

The following steps install the latest stable version of the Rust compiler.

### Installing `rustup` on Linux or macOS

If you’re using Linux or macOS, open a terminal and enter the following command:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

The command downloads a script and starts the installation of the `rustup`
tool, which installs the latest stable version of Rust. You might be prompted
for your password. If the install is successful, the following line will appear:

```text
Rust is installed now. Great!
```

You will also need a _linker_, which is a program that Rust uses to join its
compiled outputs into one file. It is likely you already have one. If you get
linker errors, you should install a C compiler, which will typically include a
linker. A C compiler is also useful because some common Rust packages depend on
C code and will need a C compiler.

On macOS, you can get a C compiler by running:

```console
$ xcode-select --install
```

Linux users should generally install GCC or Clang, according to their
distribution’s documentation. For example, if you use Ubuntu, you can install
the `build-essential` package.

### Installing `rustup` on Windows

On Windows, go to [https://www.rust-lang.org/tools/install][install] and follow
the instructions for installing Rust. At some point in the installation, you’ll
be prompted to install Visual Studio. This provides a linker and the native
libraries needed to compile programs. If you need more help with this step, see
[https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]

### Troubleshooting

To check whether you have Rust installed correctly, open a shell and enter this
line:

```console
$ rustc --version
```

You should see the version number, commit hash, and commit date for the latest
stable version that has been released, in the following format:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this information, you have installed Rust successfully! If you don’t
see this information, check that Rust is in your `%PATH%` system variable as
follows.

In Windows CMD, use:

```console
> echo %PATH%
```

In PowerShell, use:

```powershell
> echo $env:Path
```

In Linux and macOS, use:

```console
$ echo $PATH
```

If that’s all correct and Rust still isn’t working, there are a number of
places you can get help. Find out how to get in touch with other Rustaceans (a
silly nickname we call ourselves) on [the community page][community].

### Updating and Uninstalling

Once Rust is installed via `rustup`, updating to a newly released version is
easy. From your shell, run the following update script:

```console
$ rustup update
```

To uninstall Rust and `rustup`, run the following uninstall script from your
shell:

```console
$ rustup self uninstall
```

---

## Mision 3: Hands-on experience with Rust exercises

[Please read the instruction for this section here.](https://github.com/openguild-labs/open-hack-rust-starter/tree/main/src/basic)

---

## Mision 4: One Billion Row Challenge

[Please read the instruction for this section here.](https://github.com/openguild-labs/open-hack-rust-starter/tree/main/src/advanced)

# 🙌 How to contribute to the community?

To submit a proposal, ideas, or any questions, please submit them here: [OpenGuild Discussion 💬](https://github.com/orgs/openguild-labs/discussions)
View tickets and activities that you can contribute: [Community Activities 🖐️](https://github.com/orgs/openguild-labs/discussions/categories/activities)

- **Help to grow the community:** Community growth is a collective effort. By actively engaging with and inviting fellow enthusiasts to join our community, you play a crucial role in expanding our network. Encourage discussions, share valuable insights, and foster a welcoming environment for newcomers.

- **Participate in workshops and events:** Be an active participant in our workshops and events. These sessions serve as valuable opportunities to learn, collaborate, and stay updated on the latest developments in the Polkadot ecosystem. Through participation, you not only enhance your knowledge but also contribute to the collaborative spirit of OpenGuild. Share your experiences, ask questions, and forge connections with like-minded individuals.

- **Propose project ideas:** Your creativity and innovation are welcomed at OpenGuild. Propose project ideas that align with the goals of our community. Whether it's a new application, a tool, or a solution addressing a specific challenge in the Polkadot ecosystem, your ideas can spark exciting collaborations.

- **Contribute to our developer tools:** Get involved in the ongoing development and improvement of tools that aid developers in their projects. Whether it's through code contributions, bug reports, or feature suggestions, your involvement in enhancing these tools strengthens the foundation for innovation within OpenGuild and the broader Polkadot community.
