import plotter as plo
import itertools

def analyse_all(consumer_data, producer_data):
    
    analyse_round(consumer_data, producer_data)

    # for consumer_round, producer_round in zip(consumer_data, producer_data):
    #   analyse_round(consumer_round, producer_round)

def analyse_round(consumer_round, producer_round):
    
    consumers = []
    producers = []

    for consumer in consumer_round:
        consumers.append((
            consumer["max_price"],
            consumer["purchase_price"],
            consumer["purchased"],
        ))
    
    for producer in producer_round:
        producers.append((
            producer["min_price"],
            producer["selling_price"],
            producer["sold"],
        ))
    
    consumers.sort(key=lambda x: -x[0])
    producers.sort(key=lambda x:  x[0])

    plo.plt_market_round(consumers, producers)