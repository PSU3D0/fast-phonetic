<a name="readme-top"></a>


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h3 align="center">fast-phonetic</h3>

  <p align="center">
    Fast Phonetic algoritihms in Python, using Rust and rphonetic
    <br />
    <br />
    <br />
    ·
    <a href="https://github.com/PSU3D0/fast-phonetic/issues">Report Bug</a>
    ·
    <a href="https://github.com/PSU3D0/fast-phonetic/issues">Request Feature</a>
  </p>
</div>


<!-- ABOUT THE PROJECT -->
## About The Project

All the existing libraries I saw in Python seemed dead. They also seemed to be lacking in modern CI practices. Plus Rust is cool

Currently we have

* Support for all `rphonetic` encoders besides BeiderMorse and DanitchMokotoff




<!-- GETTING STARTED -->
## Getting Started


### Installation

`pip install fast_phonetic`



<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

```python
import fast_phonetic

# Soundex
soundex = fast_phonetic.encode_word("foo", "soundex")
metaphone = fast_phonetic.encode_word("foo", "metaphone")
double_metaphone = fast_phonetic.encode_word("foo", "double_metaphone")

bad_value = fast_phonetic.encode_word("foo", "bleh-encoder") # Raises NotImplemented error

```


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Add Tests
- [ ] Add auto docs
- [ ] Improved Multi-language support


See the [open issues](https://github.com/PSU3D0/fast-phonetic/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>
