import matplotlib.pyplot as plt
import numpy as np

def plt_market_round(data):
    
    actors             = np.array(list(range(0, len(data))))
    consumer_max_price = [ x[0] for x in data ]
    producer_min_price = [ x[1] for x in data ]
    traded             = [ x[2] for x in data ]

    ax = plt.subplot(111)

    ax.bar(actors - 0.25, consumer_max_price, width=0.5)
    ax.bar(actors + 0.25, producer_min_price, width=0.5)

    plt.show()