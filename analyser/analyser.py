import plotter as plo
import itertools

def analyse_all(consumer_data, producer_data):
    
    for consumer_round, producer_round in zip(consumer_data, producer_data):
        analyse_round(consumer_round, producer_round)

def analyse_round(consumer_round, producer_round):

    number_of_consumers = len(consumer_round)
    number_of_producers = len(producer_round)
    
    consumer_result = []
    producer_result = []

    

    for consumer in consumer_round:
        consumer_id = consumer["identifier"]
        purchased = consumer["purchased"]
        max_price = consumer["max_price"]

        consumer_result.append((max_price, purchased))
    
    for producer in producer_round:
        producer_id = producer["identifier"]
        sold = producer["sold"]
        min_price = producer["min_price"]

        producer_result.append((min_price, sold))


    consumer_result.sort(key=lambda x: -x[0])
    producer_result.sort(key=lambda x:  x[0])


    round_res = []
    # Assumption: if consumer get bought, then producer sold when lists are zipped. TODO
    for consumer_res, producer_res in list(itertools.zip_longest(consumer_result, producer_result, fillvalue=(0, False))):
        round_res.append((consumer_res[0], producer_res[0], consumer_res[1]))

    plo.plt_market_round(round_res)