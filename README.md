# Data Wrangling with Rust

A series of articles that explore working with data using DataFusion and Apache Arrow. 

## Table of Contents

* [Introduction to Apache Arrow and DataFusion](./introduction_to_apache_arrow_and_datafusion)
* [Data Aggregation with DataFusion](./data_aggregation_with_datafusion)
* [Data in Flight](./data_in_flight)
* [Extending DataFusion](./extending_datafusion)

## Overview

In general, the first two articles provide high level overviews, while the second two article articles take a deeper look into the libraries.

### Introduction to Apache Arrow with Rust

This article gives an overview of Apache Arrow. The focus is introducing the library and exploring its primary use cases. Important features are explored at a high level, and code samples are used to provide examples.

### Data Aggregation with DataFusion

This article provides a more in-depth look at DataFusion. The focus is on exploring how to use DataFusion to aggregate data by using either the DataFrame API or the SQL Parser API. Both methods are described in detail and illustrated with code samples.

### Data in Arrow Flight

This article describes how to use Arrow Flight to move data accross a network. The focus is on building an Arrow Flight server and client to move aggregated from server to client. Methods that are described in *Data Aggregation with DataFusion* will be used in this article. Arrow Flight will be discussed in detail  while building out a small example project.

### Extending DataFusion with an XML Reader

This article describes how DataFusion can be extended to allow for data sources that are not currently supported. The focus is on implementing and XML file reader that can be utilized with DataFusion and that reads data into the Apache Arrow memory model. In-depth features of both DataFusion and Apache Arrow will explored and described in detail while building the example project.
