from data_selector import DataSelector
from data_plotter import DataPlotter

from ipywidgets import HBox, HTML, Layout, VBox


def make_dashboard(data_manager, selector_width="350px", fig_layout=None):
    selector = DataSelector(data_manager=data_manager)
    plotter = DataPlotter(data_manager=data_manager)

    selector.observe(
        lambda _: plotter.update_traces(*selector.selections()), names=["request_plot"]
    )

    fig = plotter.fig

    # pass any user-provided layout settings
    if fig_layout is not None:
        fig.update_layout(fig_layout)

    return HBox(
        [
            VBox(
                [
                    HTML(value="<b>Select Data:</b>"),
                    selector.name_selector,
                    selector.type_selector,
                    selector.loc_selector,
                    selector.kw_selector,
                    HTML(value="<hr>"),
                    selector.ref_selector,
                    selector.plot_deltas,
                ],
                layout=Layout(height="auto", width=selector_width),
            ),
            fig,
        ]
    )
