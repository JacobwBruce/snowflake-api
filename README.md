<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->

<a name="readme-top"></a>

<br />
<div align="center">
<h3 align="center">VIC - Snowflake API Integration </h3>

  <p align="center">
    <br />
    <br />
    <a href="https://github.com/American-Expediting/vic-snowflake/issues">Report Bug</a>
    ·
    <a href="https://github.com/American-Expediting/vic-snowflake/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

Rust server built to expose Snowflake endpoints for use in VIC

<!-- GETTING STARTED -->

## Getting Started

### Prerequisites

You must have Rust installed

- Rust installation

  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Installation

Follow the instructions below to get a local development server up and running.

1. Clone the repo
   ```sh
   git clone https://github.com/American-Expediting/vic-snowflake.git
   ```
2. Set a DATABASE_URL environment variabel
   ```sh
   export DATABASE_URL='<your-db-url>'
   ```
3. Set a ACCESS_KEY environment variabel
   ```sh
   export ACCESS_KEY='<your-access-key>'
   ```
4. Run the project to install packages
   ```sh
   cargo run
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Jacob Bruce - j.bruce@amexpediting.com

<p align="right">(<a href="#readme-top">back to top</a>)</p>
