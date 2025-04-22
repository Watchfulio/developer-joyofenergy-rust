# Welcome to PowerDale

PowerDale is a small town with around 100 residents. Most houses have a smart meter installed that can save and send information about how much power a house is drawing/using.

There are three major providers of energy in town that charge different amounts for the power they supply.

- Dr Evil's Dark Energy
- The Green Eco
- Power for Everyone

## Introducing JOI Energy

JOI Energy is a new start-up in the energy industry. Rather than selling energy they want to differentiate themselves from the market by recording their customers' energy usage from their smart meters and recommending the best supplier to meet their needs.

You have been placed into their development team, whose current goal is to produce an API which their customers and smart meters will interact with.

Unfortunately, two members of the team are on annual leave, and another one has called in sick! You are left with another ThoughtWorker to progress with the current user stories on the story wall. This is your chance to make an impact on the business, improve the code base and deliver value.

### Example data

These values are used in the code and in the following examples too.

#### Users

To trial the new JOI software 5 people from the JOI accounts team have agreed to test the service and share their energy data.

| User |	Smart Meter ID |	Power Supplier |
| --- | --- | --- |
| Sarah |	smart-meter-0| 	Dr Evil's Dark Energy |
|Peter|	smart-meter-1|	The Green Eco|
|Charlie|	smart-meter-2|	Dr Evil's Dark Energy|
|Andrea	|smart-meter-3	  |Power for Everyone|
|Alex	|smart-meter-4	| The Green Eco|


#### Suppliers 

| Supplier Name        | Supplier ID  | Unit Rate |
| -------------------- | ------------ | --------- |
| Dr Evils Dark Energy | price-plan-0 | 10.0      |
| The Green Eco        | price-plan-1 | 2.0       |
| Power for Everyone   | price-plan-2 | 1.0       |

#### Energy readings

The following are sampled readings from `smart-meter-0`, in kW, every minute. Note that the reading is in kW and not kWH, which means that each reading represents the consumption at the reading time. If no power is being consumed at the time of reading, then the reading value will be 0. Given that 0 may introduce new challenges, we can assume that there is always some consumption, and we will never have a 0 reading value. These readings are then sent by the smart meter to the application using REST.

| Date (GMT)       | 	Epoch timestamp | 	Reading (kW) |
|------------------|------------------|---------------|
| 2020-11-29 8:00  | 	1606636800      | 	0.0503       |
| 2020-11-29 8:01  | 	1606636860      | 	0.0621       |
| 2020-11-29 8:02  | 1606636920	      | 0.0222        |
| 2020-11-29 8:03  | 	1606636980	     | 0.0423        |
| 2020-11-29 8:04  | 1606637040	      | 0.0191        |


### Story Wall

At JOI energy the development team use a story wall or Kanban board to keep track of features or "stories" as they are worked on.

The wall you will be working from today has 7 columns:

- Backlog
- Ready for Dev
- In Dev
- Ready for Testing
- In Testing
- Ready for sign off
- Done

Examples of Kanban boards can be found [here](https://www.planview.com/resources/guide/introduction-to-kanban/kanban-examples/)

## JOI Software

### Requirements
This project is built using Rust version 1.83.0

### Running the application

To run the application, you need to have Rust installed on your machine. You can install Rust by following the instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can run the application by executing the following command in the root directory of the project:

```
$ cargo run
```

This will start the application on port 8080. You can access the API endpoints by sending HTTP requests to the application.

### Running the tests

To run the tests, you can execute the following command in the root directory of the project:

```
$ cargo test
```

This will run all the tests in the project and output the results to the console.

### Code structure

The code is structured into the following modules:

- `src`: Contains the source code for the application.
  - `main.rs`: Contains the main entry point for the application.
  - `http`: Contains the route definitions for the application.
  - `datastore`: Contains the storage services used in the application.
- `Cargo.toml`: Contains the dependencies and metadata for the application.
- `README.md`: Contains the documentation for the application.


## API Endpoints 
Below is a list of API endpoints. Please note that the application needs to be running for the following endpoints to work. For more information about how to run the application, please refer to [Running the application](#running-the-application).

### Storing energy readings
___

Add energy readings for a smart meter.

```
POST /readings/create
```

#### Request body

**smart_meter_id** | _String_ 

 ID string for the smart meter whose readings are being stored.

**electricity_readings** | _List_ 

List of ElectricityReadings. The electricity readings that are being stored

**ElectricityReadings**

**time** | _String_

Timestamp in which the reading is taken

**reading** | _Float_

The consumption in kW at the time of the reading


```
{
    "smart_meter_id":"smart-meter-0",
    "electricity_readings":[
        {"time":"2020-11-29T08:00:00Z","reading":0.0503},
        {"time":"2020-11-29T08:01:00Z","reading":0.0621},
        {"time":"2020-11-29T08:02:00Z","reading":0.0222},
        {"time":"2020-11-29T08:03:00Z","reading":0.0423},
        {"time":"2020-11-29T08:04:00Z","reading":0.0191}
    ]
}
```
#### Example request

```    
curl \
    -X POST \
    -H "Content-Type: application/json" \
    "http://localhost:8080/readings/store" \
    -d '{"smart_meter_id":"smart-meter-0","electricity_readings":[{"time":"2020-11-29T08:00:00Z","reading":0.0503},{"time":"2020-11-29T08:01:00Z","reading":0.0621},{"time":"2020-11-29T08:02:00Z","reading":0.0222},{"time":"2020-11-29T08:03:00Z","reading":0.0423},{"time":"2020-11-29T08:04:00Z","reading":0.0191}]}'
```


#### Returns
```
Readings created successfully
```


### Getting stored readings
___

Returns a list of all the stored energy readings for the given `smart_meter_id`

```
GET /readings/read/<smart_meter_id>
```

#### Parameters

**smart_meter_id** | _String_ 

 ID string for the smart meter whose readings are being stored.

#### Example request

```
curl "http://localhost:8080/readings/read/smart-meter-0"
```

#### Returns
```
[
    {
        "time":"2020-11-29T08:00:00Z",
        "reading":0.0503
    },
    {
        "time":"2020-11-29T08:01:00Z",
        "reading":0.0621
    },
    {
        "time":"2020-11-29T08:02:00Z",
        "reading":0.0222
    },
    {
        "time":"2020-11-29T08:03:00Z",
        "reading":0.0423
    },
    {
        "time":"2020-11-29T08:04:00Z",
        "reading":0.0191
    }
]
```

### Get current Price Plan and Cost of Usage Comparisons
___

Given a `smart_meter_id` return the ID of it's current price plan, along with a comparison of the cost of usage of all the available price plans.

The price plan comparison consists of a hashmap with key value pairs of `price-plan-id` and average cost per hour based on all of the stored readings.

```
GET /price-plans/compare-all/<smart_meter_id>
```

#### Parameters

**smart_meter_id** | _String_ 

ID string for the smart meter whose readings are being stored.

#### Example request

```
curl "http://localhost:8080/price_plans/compare_all/smart-meter-0"
```

#### Returns

```
{
    "price_plans": {
        "price-plan-0": 5.88,
        "price-plan-1": 1.176,
        "price-plan-2": 0.588
    },
    "supplier_id":"price-plan-0"
}
```

### Get recommended price plans for usage
___

Given a `smart_meter_id` return a list with the recommended price plan. The top recommended price plan with be the most cost effective plan.

```
GET /price-plans/recommend/<smartMeterId>[?limit=<limit>]
```

#### Parameters

**smart_meter_id** | _String_ 

ID string for the smart meter whose readings are being stored.

**limit** | _Int_

The maximum number of recommendations that should be returned.

#### Example request

```
curl "http://localhost:8080/price_plans/recommend/smart-meter-0?limit=2"
```

#### Returns

```
[
    {
        "price-plan-2": 0.588
    },
    {
        "price-plan-1": 1.176
    }
]
```
