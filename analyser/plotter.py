import matplotlib.pyplot as plt
import numpy as np

def plt_market_round(consumer_data, producer_data):
    
    # 0: min/max
    # 1: price
    # 2: has purchased?
    # 3: Expected price

    x_axis_length = max(len(consumer_data), len(producer_data))

    actors             = np.array(list(range(0, x_axis_length)))

    consumer_max_price = [ x[0] for x in consumer_data ]
    consumer_price     = [ x[1] for x in consumer_data ]
    consumer_purchased = [ x[2] for x in consumer_data ]
    consumer_expected  = [ x[3] for x in consumer_data ]
    
    producer_min_price = [ x[0] for x in producer_data ]
    producer_price     = [ x[1] for x in producer_data ]
    producer_sold      = [ x[2] for x in producer_data ]
    producer_expected  = [ x[3] for x in producer_data ]

    print(producer_expected)

    avrage_consumer_price = np.average([ x for x in consumer_price if x != 0 ])
    avrage_producer_price = np.average([ x for x in producer_price if x != 0 ])
    # avrage_consumer_price = np.average([ consumer_price)


    consumer_surplus = np.sum([ x[0] - x[1] if x[2] == True else 0 for x in consumer_data ])
    producer_surplus = np.sum([ x[1] - x[0] if x[2] == True else 0 for x in producer_data ])

    ratio = consumer_surplus / producer_surplus

    print(f"{consumer_surplus}x{producer_surplus} (x{ratio})")

    consumer_max_price = np.pad(consumer_max_price, (0, x_axis_length - len(consumer_max_price)), 'constant')
    producer_min_price = np.pad(producer_min_price, (0, x_axis_length - len(producer_min_price)), 'constant')
    consumer_purchased = np.pad(consumer_purchased, (0, x_axis_length - len(consumer_purchased)), 'constant')
    consumer_price     = np.pad(consumer_price,     (0, x_axis_length - len(consumer_price)    ), 'constant')
    producer_sold      = np.pad(producer_sold,      (0, x_axis_length - len(producer_sold)     ), 'constant')
    producer_price     = np.pad(producer_price,     (0, x_axis_length - len(producer_price)    ), 'constant')

    ax = plt.subplot()

    consumer_col = np.where(consumer_purchased, 'navy', 'cornflowerblue')
    consumer_col_light = [ lighten_color(x) for x in consumer_col ]

    producer_col = np.where(producer_sold, 'darkorange', 'orange')
    producer_col_light = [ lighten_color(x) for x in producer_col ]

    ax.bar(actors - 0.25, consumer_max_price, width=0.5, color=consumer_col)
    ax.bar(actors - 0.25, consumer_expected,  width=0.5, color="dodgerblue")
    ax.bar(actors + 0.25, producer_expected,  width=0.5, color=lighten_color("orange"))
    ax.bar(actors + 0.25, producer_min_price, width=0.5, color=producer_col)

    ax.axhline(y=avrage_consumer_price, linestyle="--")

    plt.show()

def lighten_color(color, amount=0.5):
    """
    Lightens the given color by multiplying (1-luminosity) by the given amount.
    Input can be matplotlib color string, hex string, or RGB tuple.

    Examples:
    >> lighten_color('g', 0.3)
    >> lighten_color('#F034A3', 0.6)
    >> lighten_color((.3,.55,.1), 0.5)
    """
    import matplotlib.colors as mc
    import colorsys
    try:
        c = mc.cnames[color]
    except:
        c = color
    c = colorsys.rgb_to_hls(*mc.to_rgb(c))
    return colorsys.hls_to_rgb(c[0], 1 - amount * (1 - c[1]), c[2])