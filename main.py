import os
from pyo3_rust import Matcher, Clipboard
import tkinter as tk
from tkinter import ttk
from typing import Any, Callable, List, Optional
import sys
from threading import Timer


def main():
    matcher = Matcher("")
    clipboard = Clipboard()

    def make_handler(file: str) -> Callable[[Any], None]:
        def handler(_):
            clipboard.set_from_path(os.path.join("./", file))

        return handler

    class LabelFrame(tk.Frame):
        __list_label: List[str]
        __master: Optional[tk.Misc]

        def __init__(self, master: Optional[tk.Misc] = None):
            self.__list_label = []
            self.__master = master
            super().__init__(self.__master)

        def update_view(self, items: List[str]):
            diff = len(self.__list_label) - len(items)
            needs_to_add = diff < 0
            needs_to_delete = diff > 0
            diff = abs(diff)
            self.__list_label = items
            if needs_to_delete:
                for x, _ in zip(self.winfo_children(), range(diff)):
                    x.destroy()
            elif needs_to_add:
                for _ in range(diff):
                    x = tk.Label(self, text="").pack(pady="10px")
            children = self.winfo_children()
            if len(children):
                for child, item in zip(children, self.__list_label):
                    child.config(text=item)
                    child.bind(
                        "<Button-1>",
                        make_handler(item),
                    )
                self.pack(fill="x", padx="50px", pady="10px")
            else:
                self.pack_forget()

    class DebouncedStringVar(tk.StringVar):
        __debounce_time: float = 0
        __callblack: Callable[[str], None]
        __timer: Timer
        __enabled: bool = False

        def __init__(
            self,
            debounce_time: float,
            extra_callback: Callable[[str], None],
            master: tk.Misc | None = None,
            value: str | None = None,
            name: str | None = None,
        ) -> None:
            self.__debounce_time = debounce_time if debounce_time > 0 else 0
            self.__callblack = extra_callback
            self.__timer = Timer(self.__debounce_time, self.__callblack, args=("",))
            super().__init__(master, value, name)

        def enable(self):
            self.__enabled = True

        def disable(self):
            self.__enabled = False

        def on_change(self, *_):
            if not self.__enabled:
                return
            self.__timer.cancel()
            self.__timer = Timer(
                self.__debounce_time, self.__callblack, args=(self.get(),)
            )
            self.__timer.start()

    win = tk.Tk()
    width = 600  # Width
    height = 600  # Height

    screen_width = win.winfo_screenwidth()  # Width of the screen
    screen_height = win.winfo_screenheight()  # Height of the screen

    # Calculate Starting X and Y coordinates for Window
    x = (screen_width / 2) - (width / 2)
    y = (screen_height / 2) - (height / 2)

    # Set the geometry of window
    win.geometry("%dx%d+%d+%d" % (width, height, x, y))
    examples = os.listdir(".")
    labels = LabelFrame(win)

    def add_elements(search: str):
        matcher.set_pattern(search)
        labels.update_view(matcher.match_list_top_n(examples, 3))

    entry_text = DebouncedStringVar(0.2, add_elements)

    entry_text.trace_add("read", entry_text.on_change)
    entry_text.trace_add("write", entry_text.on_change)

    e1 = ttk.Entry(
        win, justify=tk.CENTER, textvariable=entry_text, font=("courier", 15)
    )
    e1.pack(fill="x", padx="50px", pady="10px")
    entry_text.enable()

    # Add a background color to the Main Window
    win.config(bg="#add123")

    def close(_: tk.Event):
        win.withdraw()
        sys.exit()

    win.bind("<Escape>", close)

    # Create a transparent window
    win.wm_attributes("-transparentcolor", "#add123")
    win.wm_attributes("-topmost", True)
    win.overrideredirect(True)
    win.mainloop()


if __name__ == "__main__":
    main()
