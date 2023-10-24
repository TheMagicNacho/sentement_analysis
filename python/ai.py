from transformers import pipeline
import torch
import docx

hf_name = 'pszemraj/led-base-book-summary'


def summarizer(input_text):
    try:
        summarizer = pipeline(
            "summarization",
            hf_name,
            device=0 if torch.cuda.is_available() else -1,
        )

        result = summarizer(
            input_text,
            min_length=16,
            max_length=225,
            no_repeat_ngram_size=3,
            encoder_no_repeat_ngram_size=3,
            repetition_penalty=3.5,
            num_beams=4,
            early_stopping=True,
        )

        return result
    except Exception as e:
        return "Python3: Failed to Run: " + str(e)


def run(input_file):
    try:
        output = ""
        if input_file.split('.')[-1] == 'docx':
            file = docx.Document(input_file)
            req = ''
            paragraphs = file.paragraphs
            for para in paragraphs:
                req += para.text
 
            res = summarizer(req)
            output = res[0]['summary_text']
        elif input_file.split('.')[-1] == 'txt':
            with open(input_file, 'r') as f:
                req = f.read()
            res = summarizer(req)
            output = res[0]['summary_text']
        else:
            print('Invalid file format. Please use .docx or .txt files only.')
            output = ""

        return output
    except Exception as e:
        # print('failed to run: ', e)
        return "Python3: Failed to Run: " + str(e)
