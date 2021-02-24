# Data Wrangling with Rust

A series of articles that explore working with data using Datafusion and Apache Arrow. 

## Table of Contents

* [Introduction to Apache Arrow and Datafusion](./introduction_to_apache_arrow_and_datafusion)
* [Data Aggregation with Datafusion](./data_aggregation_with_datafusion)
* [Data in Flight](./data_in_flight)
* [Extending Datafusion](./extending_datafusion)

## Overview

In general, the first two articles provide high level overviews, while the second two article articles take a deeper look into the libraries.

### Introduction to Apache Arrow and Datafusion

This article gives an overview of both Apache Arrow and Datafusion. The focus is introducing both of the libraries and exploring their primary use cases. Important features are explored at a high level, and code samples are used to provide examples.

### Data Aggregation with Datafusion

This article provides a more in-depth look at Datfusion. The focus is on exploring how to use Datafusion to aggregate data by using either the DataFrame API or the SQL Parser API. Both methods are described in detail and illustrated with code samples.

### Data in Flight

This article describes how to use Arrow Flight to move data accross a network. The focus is on building an Arrow Flight server and client to move aggregated from server to client. Methods that are described in *Data Aggregation with Datafusion* will be used in this article. Arrow Flight will be discussed in detail  while building out a small example project.

### Extending Datafusion

This article describes how Datafusion can be extended to allow for data sources that are not currently supported. The focus is on implementing and XML file reader that can be utilized with Datafusion and that reads data into the Apache Arrow memory model. In-depth features of both Datafusion and Apache Arrow will explored and described in detail while building the example project.
