[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![Apache 2.0 License][license-shield]][license-url]



<h1 align="center">Custom Car Manager</h1>
<p align="center">
  A simple program to manage <a href="https://www.nexusmods.com/derailvalley/mods/324">Custom Car Loader</a> cars for <a href="https://store.steampowered.com/app/588030">Derail Valley</a>.
  <br />
  <br />
  <a href="https://github.com/Insprill/dv-custom-car-manager/issues">Report Bugs</a>
  ·
  <a href="https://github.com/Insprill/dv-custom-car-manager/issues">Request Features</a>
</p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#compiling">Compiling</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>




<!-- USAGE -->

## Usage

### Installation

Custom Car Manager doesn't need installation as it is a standalone program.  
The one thing you do need to have installed is Java 17 or newer. You can install that [here](https://download.oracle.com/java/17/archive/jdk-17.0.3.1_windows-x64_bin.exe).  

### Setup

1. 
    When you run CCM for the first time, you will get a warning telling you to set your Derail Valley installation directory.
    ![Picture](https://imgur.com/DKTAK0D.png)  
    You can simply press Close.


2. 
    To set your Derail Valley installation directory, click the "Select Installation Directory" button.
    ![Picture](https://imgur.com/FaijhWX.png)  
    If you're unsure where Derail Valley is installed, you can find it in Steam.
      1. Right click on Derail Valley
      2. Click "Properties"
      3. Click "Local Files"
      4. Click "Browse"
      ![Picture](https://imgur.com/CU0hGE6.png)
      The directory that opens in File Explorer is your Derail Valley installation directory.  
      You can copy this directory to your clipboard by right-clicking the "Derail Valley" in the menu bar, and clicking "Copy address".  
      ![Picture](https://imgur.com/ARKVDEZ.png)  


3. 
    Once your installation directory is set, you should see any cars you currently have installed in the "Installed Cars" section.
    ![Picture](https://imgur.com/ojvuHPx.png)

<br>

### Installing Cars

Installing cars can be done in one of two ways. They can be installed from a `.zip` file, or from a folder.  
Both methods support installing multiple cars at once.

To install cars from a folder, click the "Install Car(s) from Folder" button.  
![Picture](https://imgur.com/kwjRoA3.png)  
Now simply navigate to the folder where the car is located, and select it. CCM will now install the car.

Installing cars from `.zip` files is basically the same process, except you slect the `.zip` file instead of the folder.

<br>

### Uninstalling Cars

Uninstalling cars is as simple as pressing the `Delete` button next to the car you want to uninstall.




<!-- Compiling -->

## Compiling

To compile dv-custom-car-manager, you need JDK 17 or higher and an internet connection.  
Clone this repo, then run `./gradlew build` from your terminal.  
You can find the compiled jar in the `build/libs` directory.




<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any
contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Make your changes.
4. Stage your changes (`git add .`)
5. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
6. Push to the Branch (`git push origin feature/AmazingFeature`)
7. Open a Pull Request




<!-- LICENSE -->

## License

Distributed under the Apache 2.0 License. See [`LICENSE`][license-url] for more information.




<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/Insprill/dv-custom-car-manager.svg?style=for-the-badge
[contributors-url]: https://github.com/Insprill/dv-custom-car-manager/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Insprill/dv-custom-car-manager.svg?style=for-the-badge
[forks-url]: https://github.com/Insprill/dv-custom-car-manager/network/members
[stars-shield]: https://img.shields.io/github/stars/Insprill/dv-custom-car-manager.svg?style=for-the-badge
[stars-url]: https://github.com/Insprill/dv-custom-car-manager/stargazers
[issues-shield]: https://img.shields.io/github/issues/Insprill/dv-custom-car-manager.svg?style=for-the-badge
[issues-url]: https://github.com/Insprill/dv-custom-car-manager/issues
[license-shield]: https://img.shields.io/github/license/Insprill/dv-custom-car-manager.svg?style=for-the-badge
[license-url]: https://github.com/Insprill/dv-custom-car-manager/blob/master/LICENSE
[maven-central-shield]: https://img.shields.io/maven-central/v/net.insprill/dv-custom-car-manager
[maven-central-url]: https://mvnrepository.com/artifact/net.insprill/dv-custom-car-manager
