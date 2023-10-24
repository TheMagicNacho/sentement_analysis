# 🤖 Sentement Analysis Tool 

## 🏁 Preqequisits

- You need python 3.11 
- You need a terminal
- You need internet.
- Having a GPU is helpful

### Clone The repo

Clone this repo to your working directory.

`git clone https://github.com/TheMagicNacho/sentement_analysis.git`


## 🚀 How to use:

1. Install Requirements:

`pip install -r requirements.txt`

Conda: `conda install --file=requirements.txt`

NOTE: You only need to do this once.

2. Run the program:

`python3 main.py <input.docx>`

Note: you can replace the input file with whatever file you want


3. The summary is found in the console.


# Updates
- if more resources are available update the model: pszemraj/led-large-book-summary
- Ref: [https://huggingface.co/docs/transformers/installation](https://huggingface.co/docs/transformers/installation)
- Ref:
    [https://huggingface.co/pszemraj/led-large-book-summary](https://huggingface.co/pszemraj/led-large-book-summary)


# Notes to Self
- When using conda environment, you need to staticly link the library: `export LD_LIBRARY_PATH=~/miniconda3/envs/env-sentement/lib`
- To install dependencies with conda set conda-forge channel to access the
    Hugging Face libraries: `conda config --env --add channels conda-forge`


