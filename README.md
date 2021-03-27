This crate / package is a rust module that handles streaming input and output. It's purpose is to tally or accumulate  values for  a streaming set of keys. It is designed to be stupidly simple and consume / produce whitespace seperate values.

# Key Value Counts

I use this library to parse simple journal-like logs where each line is of the form:

```
2021-03-01 warnings:3 error ... (other items with optional counts)
```

Supposing I wanted to do some processing on this data. This is a very readable / writeable format, but is not standard.

We can use 
`kvc-stretch` to covert it into something more lika a [stream of k-value pairs](https://www.edureka.co/blog/kafka-streams/#stream) 
or 
`kvc-df` to convert it to a [pandas dataframe](https://pandas.pydata.org/pandas-docs/stable/getting_started/intro_tutorials/01_table_oriented.html#min-tut-01-tableoriented)

# Spec

The kvc journal format is very simple.

- Each line is a "frame"
- A frame has an optional "date header"
- A frame is composed of a string of whitespace-seperated keys with optional counts per key
- A '#' ends the frame, and is useful for comments

These are valid frames, one per line:

```
a
event event
2021-04-01 april_fools_pranks:4
2021-03-01 key another_key a-third-key <weird-symbols_ar_ok!> this_has_occured_three_times:3 this_twice this_twice
2021-04-02 # Nothing happened that day
```

Running kda-stretch on the above data produces:

```
```

Running kda-df produces:

```
```

I use this to keep a journal of events and easily scrape it for analysis in other programs or databases. 
