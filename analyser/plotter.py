import matplotlib.pyplot as plt
import numpy as np

def plt_market_round(consumer_data, producer_data):
    
    x_axis_length = max(len(consumer_data), len(producer_data))

    actors             = np.array(list(range(0, x_axis_length)))
    consumer_max_price = [ x[0] for x in consumer_data ]
    producer_min_price = [ x[0] for x in producer_data ]
    consumer_purchased = [ x[2] for x in consumer_data ]
    producer_sold      = [ x[2] for x in producer_data ]

    consumer_max_price = np.pad(consumer_max_price, (0, x_axis_length - len(consumer_max_price)), 'constant')
    producer_min_price = np.pad(producer_min_price, (0, x_axis_length - len(producer_min_price)), 'constant')
    consumer_purchased = np.pad(consumer_purchased, (0, x_axis_length - len(consumer_purchased)), 'constant')
    producer_sold = np.pad(producer_sold, (0, x_axis_length - len(producer_sold)), 'constant')

    ax = plt.subplot(111)

    consumer_col = np.where(consumer_purchased, 'navy', 'cornflowerblue')
    producer_col = np.where(producer_sold, 'darkorange', 'khaki')

    ax.bar(actors - 0.25, consumer_max_price, width=0.5, color=consumer_col)
    ax.bar(actors + 0.25, producer_min_price, width=0.5, color=producer_col)

    plt.show()