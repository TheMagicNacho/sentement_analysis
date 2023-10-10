from transformers import pipeline
import torch
import docx
import sys

hf_name = 'pszemraj/led-base-book-summary'

def summarizer(input_text):
    summarizer = pipeline(
        "summarization",
        hf_name,
        device=0 if torch.cuda.is_available() else -1,
    )

    result = summarizer(
        input_text,
        min_length=16,
        max_length=256,
        no_repeat_ngram_size=3,
        encoder_no_repeat_ngram_size=3,
        repetition_penalty=3.5,
        num_beams=4,
        early_stopping=True,
    )

    return result

def main():
    input_file = sys.argv[1]
    if input_file.split('.')[-1] == 'docx':
        file = docx.Document('./input.docx')
        req = ''
        paragraphs = file.paragraphs
        for para in paragraphs:
            req += para.text
        res = summarizer(req)
        print('--Summary--')
        print(res[0]['summary_text'])
    elif input_file.split('.')[-1] == 'txt':
        with open(input_file, 'r') as f:
            req = f.read()
        res = summarizer(req)
        print('--Summary--')
        print(res[0]['summary_text'])
    else:
        print('Invalid file format. Please use .docx or .txt files only.')

if __name__ == '__main__':
    try:
        main()
    except Exception as e:
        if str(e) == 'list index out of range':
            print('Please run with a valid file. (python3 main.py <filename>)')
        elif 'CUDA out of memory' in str(e):
            print('Please use a smaller file.')
        else:
            print(e)

