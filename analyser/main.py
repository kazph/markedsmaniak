import pandas as pd
import json
import analyser as al

def get_file_content(filepath):
    with open(filepath, "r") as f:
        content = f.read()

    return content

def main():
    # get the data from the output directory
    producer_data = get_file_content("./output/producers.json")
    consumer_data = get_file_content("./output/consumers.json")
    
    producer_json = json.loads(producer_data)
    consumer_json = json.loads(consumer_data)

    # analysere dataen
    al.analyse_all(consumer_json, producer_json)

    # plotte og outpute
    pass

if __name__ == "__main__":
    main()