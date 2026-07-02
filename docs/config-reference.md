

<!DOCTYPE html>
<html lang="en">
<head>
    <link rel="stylesheet" type="text/css" href="https://fonts.googleapis.com/css?family=Overpass:300,400,600,800">
    <script src="https://code.jquery.com/jquery-3.4.1.min.js" integrity="sha256-CSXorXvZcTkaix6Yvo6HppcZGetbYMGWSFlBw8HfCJo=" crossorigin="anonymous"></script>
    <link href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T" crossorigin="anonymous">
    <script src="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/js/bootstrap.min.js" integrity="sha384-JjSmVgyd0p3pXB1rRibZUAYoIIy6OrQ6VrjIEaFf/nJGzIxFDsf4x0xIM+B07jRM" crossorigin="anonymous"></script>
	<link rel="stylesheet" type="text/css" href="schema_doc.css">
    <script src="https://use.fontawesome.com/facf9fa52c.js"></script>
    <script src="schema_doc.min.js"></script>
    <meta charset="utf-8"/>


    <title>Setlist</title>
</head>
<body onload="anchorOnLoad();" id="root">

    <div class="breadcrumbs"></div> <h1>Setlist</h1><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>A setlist file containing one or more presets and optional global config.</p>
</span>






<div class="accordion" id="accordionglobal">
    <div class="card">
        <div class="card-header" id="headingglobal">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#global"
                        aria-expanded="" aria-controls="global" onclick="setAnchor('#global')"><span class="property-name">global</span></button>
            </h2>
        </div>

        <div id="global"
             class="collapse property-definition-div" aria-labelledby="headingglobal"
             data-parent="#accordionglobal">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a></div><br/>
<span class="description"><p>Global device settings (MIDI routing, clock, etc.). Applied once on upload.</p>
</span><div class="any-of-value" id="global_anyOf"><h2 class="handle">
  <label>Any of</label>
</h2><ul class="nav nav-tabs" id="tabsglobal_anyOf_anyOf" role="tablist"><li class="nav-item">
            <a class="nav-link active anyOf-option"
               id="global_anyOf_i0" data-toggle="tab" href="#tab-pane_global_anyOf_i0" role="tab"
               onclick="setAnchor('#global_anyOf_i0')"
            >GlobalYamlConfig</a>
        </li><li class="nav-item">
            <a class="nav-link anyOf-option"
               id="global_anyOf_i1" data-toggle="tab" href="#tab-pane_global_anyOf_i1" role="tab"
               onclick="setAnchor('#global_anyOf_i1')"
            >Option 2</a>
        </li></ul>
<div class="tab-content card"><div class="tab-pane fade card-body active show"
             id="tab-pane_global_anyOf_i0" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">GlobalYamlConfig</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Global device configuration — system-wide settings independent of presets.</p>
</span>








<div class="accordion" id="accordionglobal_anyOf_i0_bpm">
    <div class="card">
        <div class="card-header" id="headingglobal_anyOf_i0_bpm">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#global_anyOf_i0_bpm"
                        aria-expanded="" aria-controls="global_anyOf_i0_bpm" onclick="setAnchor('#global_anyOf_i0_bpm')"><span class="property-name">bpm</span></button>
            </h2>
        </div>

        <div id="global_anyOf_i0_bpm"
             class="collapse property-definition-div" aria-labelledby="headingglobal_anyOf_i0_bpm"
             data-parent="#accordionglobal_anyOf_i0_bpm">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_bpm" onclick="anchorLink('global_anyOf_i0_bpm')">bpm</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint16</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>MIDI Clock tempo in BPM (30-300). Default: 120.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionglobal_anyOf_i0_calibration">
    <div class="card">
        <div class="card-header" id="headingglobal_anyOf_i0_calibration">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#global_anyOf_i0_calibration"
                        aria-expanded="" aria-controls="global_anyOf_i0_calibration" onclick="setAnchor('#global_anyOf_i0_calibration')"><span class="property-name">calibration</span></button>
            </h2>
        </div>

        <div id="global_anyOf_i0_calibration"
             class="collapse property-definition-div" aria-labelledby="headingglobal_anyOf_i0_calibration"
             data-parent="#accordionglobal_anyOf_i0_calibration">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration" onclick="anchorLink('global_anyOf_i0_calibration')">calibration</a></div><br/>
<span class="description"><p>Expression pedal ADC calibration values.</p>
</span><div class="any-of-value" id="global_anyOf_i0_calibration_anyOf"><h2 class="handle">
  <label>Any of</label>
</h2><ul class="nav nav-tabs" id="tabsglobal_anyOf_i0_calibration_anyOf_anyOf" role="tablist"><li class="nav-item">
            <a class="nav-link active anyOf-option"
               id="global_anyOf_i0_calibration_anyOf_i0" data-toggle="tab" href="#tab-pane_global_anyOf_i0_calibration_anyOf_i0" role="tab"
               onclick="setAnchor('#global_anyOf_i0_calibration_anyOf_i0')"
            >CalibrationYaml</a>
        </li><li class="nav-item">
            <a class="nav-link anyOf-option"
               id="global_anyOf_i0_calibration_anyOf_i1" data-toggle="tab" href="#tab-pane_global_anyOf_i0_calibration_anyOf_i1" role="tab"
               onclick="setAnchor('#global_anyOf_i0_calibration_anyOf_i1')"
            >Option 2</a>
        </li></ul>
<div class="tab-content card"><div class="tab-pane fade card-body active show"
             id="tab-pane_global_anyOf_i0_calibration_anyOf_i0" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration" onclick="anchorLink('global_anyOf_i0_calibration')">calibration</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0')">CalibrationYaml</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>ADC calibration for expression pedals.</p>
</span>








<div class="accordion" id="accordionglobal_anyOf_i0_calibration_anyOf_i0_exp1">
    <div class="card">
        <div class="card-header" id="headingglobal_anyOf_i0_calibration_anyOf_i0_exp1">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#global_anyOf_i0_calibration_anyOf_i0_exp1"
                        aria-expanded="" aria-controls="global_anyOf_i0_calibration_anyOf_i0_exp1" onclick="setAnchor('#global_anyOf_i0_calibration_anyOf_i0_exp1')"><span class="property-name">exp1</span></button>
            </h2>
        </div>

        <div id="global_anyOf_i0_calibration_anyOf_i0_exp1"
             class="collapse property-definition-div" aria-labelledby="headingglobal_anyOf_i0_calibration_anyOf_i0_exp1"
             data-parent="#accordionglobal_anyOf_i0_calibration_anyOf_i0_exp1">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration" onclick="anchorLink('global_anyOf_i0_calibration')">calibration</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1')">exp1</a></div><br/>
<span class="description"><p>Expression pedal 1 calibration.</p>
</span><div class="any-of-value" id="global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf"><h2 class="handle">
  <label>Any of</label>
</h2><ul class="nav nav-tabs" id="tabsglobal_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_anyOf" role="tablist"><li class="nav-item">
            <a class="nav-link active anyOf-option"
               id="global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0" data-toggle="tab" href="#tab-pane_global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0" role="tab"
               onclick="setAnchor('#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0')"
            >ExpCalibration</a>
        </li><li class="nav-item">
            <a class="nav-link anyOf-option"
               id="global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i1" data-toggle="tab" href="#tab-pane_global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i1" role="tab"
               onclick="setAnchor('#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i1')"
            >Option 2</a>
        </li></ul>
<div class="tab-content card"><div class="tab-pane fade card-body active show"
             id="tab-pane_global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration" onclick="anchorLink('global_anyOf_i0_calibration')">calibration</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1')">exp1</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0')">ExpCalibration</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Min/max ADC values for a single expression pedal (0-4095).</p>
</span>








<div class="accordion" id="accordionglobal_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_max">
    <div class="card">
        <div class="card-header" id="headingglobal_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_max">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_max"
                        aria-expanded="" aria-controls="global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_max" onclick="setAnchor('#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_max')"><span class="property-name">max</span></button>
            </h2>
        </div>

        <div id="global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_max"
             class="collapse property-definition-div" aria-labelledby="headingglobal_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_max"
             data-parent="#accordionglobal_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_max">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration" onclick="anchorLink('global_anyOf_i0_calibration')">calibration</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1')">exp1</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_max" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_max')">max</a></div><span class="badge badge-dark value-type">Type: integer</span><span class="badge badge-info value-type">Format: uint16</span> <span class="badge badge-success default-value">Default: 3750</span><br/>
<span class="description"><p>ADC value at toe (full) position. Default: 3750.</p>
</span>



        <p><span class="badge badge-light restriction numeric-restriction" id="global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_max_number">Value must be greater or equal to <code>0.0</code></span></p>


            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionglobal_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_min">
    <div class="card">
        <div class="card-header" id="headingglobal_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_min">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_min"
                        aria-expanded="" aria-controls="global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_min" onclick="setAnchor('#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_min')"><span class="property-name">min</span></button>
            </h2>
        </div>

        <div id="global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_min"
             class="collapse property-definition-div" aria-labelledby="headingglobal_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_min"
             data-parent="#accordionglobal_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_min">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration" onclick="anchorLink('global_anyOf_i0_calibration')">calibration</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1')">exp1</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_min" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_min')">min</a></div><span class="badge badge-dark value-type">Type: integer</span><span class="badge badge-info value-type">Format: uint16</span> <span class="badge badge-success default-value">Default: 0</span><br/>
<span class="description"><p>ADC value at heel (rest) position. Default: 0.</p>
</span>



        <p><span class="badge badge-light restriction numeric-restriction" id="global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0_min_number">Value must be greater or equal to <code>0.0</code></span></p>


            </div>
        </div>
    </div>
</div>
        </div><div class="tab-pane fade card-body "
             id="tab-pane_global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i1" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration" onclick="anchorLink('global_anyOf_i0_calibration')">calibration</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1')">exp1</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i1" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i1')">item 1</a></div><span class="badge badge-dark value-type">Type: null</span><br/>







        </div></div></div>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionglobal_anyOf_i0_calibration_anyOf_i0_exp2">
    <div class="card">
        <div class="card-header" id="headingglobal_anyOf_i0_calibration_anyOf_i0_exp2">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#global_anyOf_i0_calibration_anyOf_i0_exp2"
                        aria-expanded="" aria-controls="global_anyOf_i0_calibration_anyOf_i0_exp2" onclick="setAnchor('#global_anyOf_i0_calibration_anyOf_i0_exp2')"><span class="property-name">exp2</span></button>
            </h2>
        </div>

        <div id="global_anyOf_i0_calibration_anyOf_i0_exp2"
             class="collapse property-definition-div" aria-labelledby="headingglobal_anyOf_i0_calibration_anyOf_i0_exp2"
             data-parent="#accordionglobal_anyOf_i0_calibration_anyOf_i0_exp2">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration" onclick="anchorLink('global_anyOf_i0_calibration')">calibration</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp2" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp2')">exp2</a></div><br/>
<span class="description"><p>Expression pedal 2 calibration.</p>
</span><div class="any-of-value" id="global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf"><h2 class="handle">
  <label>Any of</label>
</h2><ul class="nav nav-tabs" id="tabsglobal_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_anyOf" role="tablist"><li class="nav-item">
            <a class="nav-link active anyOf-option"
               id="global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_i0" data-toggle="tab" href="#tab-pane_global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_i0" role="tab"
               onclick="setAnchor('#global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_i0')"
            >ExpCalibration</a>
        </li><li class="nav-item">
            <a class="nav-link anyOf-option"
               id="global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_i1" data-toggle="tab" href="#tab-pane_global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_i1" role="tab"
               onclick="setAnchor('#global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_i1')"
            >Option 2</a>
        </li></ul>
<div class="tab-content card"><div class="tab-pane fade card-body active show"
             id="tab-pane_global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_i0" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration" onclick="anchorLink('global_anyOf_i0_calibration')">calibration</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp2" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp2')">exp2</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_i0')">ExpCalibration</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Min/max ADC values for a single expression pedal (0-4095).</p>
</span><a href="#global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0')" class="ref-link">Same definition as global_anyOf_i0_calibration_anyOf_i0_exp1_anyOf_i0</a>
        </div><div class="tab-pane fade card-body "
             id="tab-pane_global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_i1" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration" onclick="anchorLink('global_anyOf_i0_calibration')">calibration</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp2" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp2')">exp2</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_i1" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i0_exp2_anyOf_i1')">item 1</a></div><span class="badge badge-dark value-type">Type: null</span><br/>







        </div></div></div>






            </div>
        </div>
    </div>
</div>
        </div><div class="tab-pane fade card-body "
             id="tab-pane_global_anyOf_i0_calibration_anyOf_i1" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration" onclick="anchorLink('global_anyOf_i0_calibration')">calibration</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf" onclick="anchorLink('global_anyOf_i0_calibration_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_calibration_anyOf_i1" onclick="anchorLink('global_anyOf_i0_calibration_anyOf_i1')">item 1</a></div><span class="badge badge-dark value-type">Type: null</span><br/>







        </div></div></div>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionglobal_anyOf_i0_din_enabled">
    <div class="card">
        <div class="card-header" id="headingglobal_anyOf_i0_din_enabled">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#global_anyOf_i0_din_enabled"
                        aria-expanded="" aria-controls="global_anyOf_i0_din_enabled" onclick="setAnchor('#global_anyOf_i0_din_enabled')"><span class="property-name">din_enabled</span></button>
            </h2>
        </div>

        <div id="global_anyOf_i0_din_enabled"
             class="collapse property-definition-div" aria-labelledby="headingglobal_anyOf_i0_din_enabled"
             data-parent="#accordionglobal_anyOf_i0_din_enabled">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_din_enabled" onclick="anchorLink('global_anyOf_i0_din_enabled')">din_enabled</a></div><span class="badge badge-dark value-type">Type: boolean or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Enable DIN MIDI output for locally-generated messages. Default: true.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionglobal_anyOf_i0_din_to_usb_thru">
    <div class="card">
        <div class="card-header" id="headingglobal_anyOf_i0_din_to_usb_thru">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#global_anyOf_i0_din_to_usb_thru"
                        aria-expanded="" aria-controls="global_anyOf_i0_din_to_usb_thru" onclick="setAnchor('#global_anyOf_i0_din_to_usb_thru')"><span class="property-name">din_to_usb_thru</span></button>
            </h2>
        </div>

        <div id="global_anyOf_i0_din_to_usb_thru"
             class="collapse property-definition-div" aria-labelledby="headingglobal_anyOf_i0_din_to_usb_thru"
             data-parent="#accordionglobal_anyOf_i0_din_to_usb_thru">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_din_to_usb_thru" onclick="anchorLink('global_anyOf_i0_din_to_usb_thru')">din_to_usb_thru</a></div><span class="badge badge-dark value-type">Type: boolean or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Route incoming DIN MIDI to USB MIDI out. Default: true.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionglobal_anyOf_i0_midi_clock">
    <div class="card">
        <div class="card-header" id="headingglobal_anyOf_i0_midi_clock">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#global_anyOf_i0_midi_clock"
                        aria-expanded="" aria-controls="global_anyOf_i0_midi_clock" onclick="setAnchor('#global_anyOf_i0_midi_clock')"><span class="property-name">midi_clock</span></button>
            </h2>
        </div>

        <div id="global_anyOf_i0_midi_clock"
             class="collapse property-definition-div" aria-labelledby="headingglobal_anyOf_i0_midi_clock"
             data-parent="#accordionglobal_anyOf_i0_midi_clock">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_midi_clock" onclick="anchorLink('global_anyOf_i0_midi_clock')">midi_clock</a></div><span class="badge badge-dark value-type">Type: boolean or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Enable MIDI Clock output. Default: false.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionglobal_anyOf_i0_usb_to_din_thru">
    <div class="card">
        <div class="card-header" id="headingglobal_anyOf_i0_usb_to_din_thru">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#global_anyOf_i0_usb_to_din_thru"
                        aria-expanded="" aria-controls="global_anyOf_i0_usb_to_din_thru" onclick="setAnchor('#global_anyOf_i0_usb_to_din_thru')"><span class="property-name">usb_to_din_thru</span></button>
            </h2>
        </div>

        <div id="global_anyOf_i0_usb_to_din_thru"
             class="collapse property-definition-div" aria-labelledby="headingglobal_anyOf_i0_usb_to_din_thru"
             data-parent="#accordionglobal_anyOf_i0_usb_to_din_thru">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_usb_to_din_thru" onclick="anchorLink('global_anyOf_i0_usb_to_din_thru')">usb_to_din_thru</a></div><span class="badge badge-dark value-type">Type: boolean or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Route incoming USB MIDI to DIN MIDI out. Default: false.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionglobal_anyOf_i0_usb_to_usb_thru">
    <div class="card">
        <div class="card-header" id="headingglobal_anyOf_i0_usb_to_usb_thru">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#global_anyOf_i0_usb_to_usb_thru"
                        aria-expanded="" aria-controls="global_anyOf_i0_usb_to_usb_thru" onclick="setAnchor('#global_anyOf_i0_usb_to_usb_thru')"><span class="property-name">usb_to_usb_thru</span></button>
            </h2>
        </div>

        <div id="global_anyOf_i0_usb_to_usb_thru"
             class="collapse property-definition-div" aria-labelledby="headingglobal_anyOf_i0_usb_to_usb_thru"
             data-parent="#accordionglobal_anyOf_i0_usb_to_usb_thru">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0" onclick="anchorLink('global_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i0_usb_to_usb_thru" onclick="anchorLink('global_anyOf_i0_usb_to_usb_thru')">usb_to_usb_thru</a></div><span class="badge badge-dark value-type">Type: boolean or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Route incoming USB MIDI back to USB MIDI out (echo). Default: false.</p>
</span>






            </div>
        </div>
    </div>
</div>
        </div><div class="tab-pane fade card-body "
             id="tab-pane_global_anyOf_i1" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global" onclick="anchorLink('global')">global</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf" onclick="anchorLink('global_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#global_anyOf_i1" onclick="anchorLink('global_anyOf_i1')">item 1</a></div><span class="badge badge-dark value-type">Type: null</span><br/>







        </div></div></div>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets">
    <div class="card">
        <div class="card-header" id="headingpresets">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets"
                        aria-expanded="" aria-controls="presets" onclick="setAnchor('#presets')"><span class="property-name">presets</span> <span class="badge badge-warning required-property">Required</span></button>
            </h2>
        </div>

        <div id="presets"
             class="collapse property-definition-div" aria-labelledby="headingpresets"
             data-parent="#accordionpresets">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a></div><span class="badge badge-dark value-type">Type: array</span><br/>
<span class="description"><p>List of presets. Each preset defines the complete button/encoder/expression layout for one song or scene.</p>
</span>





         <span class="badge badge-info no-additional">No Additional Items</span><h4>Each item of this array must be:</h4>
    <div class="card">
        <div class="card-body items-definition" id="presets_items">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">PresetConfig</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>A single preset — one song or scene in your setlist.</p>
</span>








<div class="accordion" id="accordionpresets_items_analog">
    <div class="card">
        <div class="card-header" id="headingpresets_items_analog">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_analog"
                        aria-expanded="" aria-controls="presets_items_analog" onclick="setAnchor('#presets_items_analog')"><span class="property-name">analog</span></button>
            </h2>
        </div>

        <div id="presets_items_analog"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_analog"
             data-parent="#accordionpresets_items_analog">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog" onclick="anchorLink('presets_items_analog')">analog</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Expression pedal configurations keyed by jack: Exp1, Exp2.</p>
</span>






<div class="accordion" id="accordionpresets_items_analog_additionalProperties">
    <div class="card">
        <div class="card-header" id="headingpresets_items_analog_additionalProperties">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_analog_additionalProperties"
                        aria-expanded="" aria-controls="presets_items_analog_additionalProperties" onclick="setAnchor('#presets_items_analog_additionalProperties')"><em><span class="property-name">Additional Properties</span></em></button>
            </h2>
        </div>

        <div id="presets_items_analog_additionalProperties"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_analog_additionalProperties"
             data-parent="#accordionpresets_items_analog_additionalProperties">
            <div class="card-body pl-5"><p class="additional-properties">Each additional property must conform to the following schema</p>

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog" onclick="anchorLink('presets_items_analog')">analog</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog_additionalProperties" onclick="anchorLink('presets_items_analog_additionalProperties')">AnalogYamlConfig</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Expression pedal (analog input) configuration.</p>
</span>








<div class="accordion" id="accordionpresets_items_analog_additionalProperties_cc">
    <div class="card">
        <div class="card-header" id="headingpresets_items_analog_additionalProperties_cc">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_analog_additionalProperties_cc"
                        aria-expanded="" aria-controls="presets_items_analog_additionalProperties_cc" onclick="setAnchor('#presets_items_analog_additionalProperties_cc')"><span class="property-name">cc</span> <span class="badge badge-warning required-property">Required</span></button>
            </h2>
        </div>

        <div id="presets_items_analog_additionalProperties_cc"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_analog_additionalProperties_cc"
             data-parent="#accordionpresets_items_analog_additionalProperties_cc">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog" onclick="anchorLink('presets_items_analog')">analog</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog_additionalProperties" onclick="anchorLink('presets_items_analog_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog_additionalProperties_cc" onclick="anchorLink('presets_items_analog_additionalProperties_cc')">cc</a></div><span class="badge badge-dark value-type">Type: integer</span><span class="badge badge-info value-type">Format: uint8</span><br/>
<span class="description"><p>MIDI CC number to send (0-127).</p>
</span>



        <p><span class="badge badge-light restriction numeric-restriction" id="presets_items_analog_additionalProperties_cc_number">Value must be greater or equal to <code>0.0</code></span></p>


            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_analog_additionalProperties_channel">
    <div class="card">
        <div class="card-header" id="headingpresets_items_analog_additionalProperties_channel">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_analog_additionalProperties_channel"
                        aria-expanded="" aria-controls="presets_items_analog_additionalProperties_channel" onclick="setAnchor('#presets_items_analog_additionalProperties_channel')"><span class="property-name">channel</span></button>
            </h2>
        </div>

        <div id="presets_items_analog_additionalProperties_channel"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_analog_additionalProperties_channel"
             data-parent="#accordionpresets_items_analog_additionalProperties_channel">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog" onclick="anchorLink('presets_items_analog')">analog</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog_additionalProperties" onclick="anchorLink('presets_items_analog_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog_additionalProperties_channel" onclick="anchorLink('presets_items_analog_additionalProperties_channel')">channel</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>MIDI channel (1-16). Default: 1.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_analog_additionalProperties_label">
    <div class="card">
        <div class="card-header" id="headingpresets_items_analog_additionalProperties_label">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_analog_additionalProperties_label"
                        aria-expanded="" aria-controls="presets_items_analog_additionalProperties_label" onclick="setAnchor('#presets_items_analog_additionalProperties_label')"><span class="property-name">label</span> <span class="badge badge-warning required-property">Required</span></button>
            </h2>
        </div>

        <div id="presets_items_analog_additionalProperties_label"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_analog_additionalProperties_label"
             data-parent="#accordionpresets_items_analog_additionalProperties_label">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog" onclick="anchorLink('presets_items_analog')">analog</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog_additionalProperties" onclick="anchorLink('presets_items_analog_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog_additionalProperties_label" onclick="anchorLink('presets_items_analog_additionalProperties_label')">label</a></div><span class="badge badge-dark value-type">Type: string</span><br/>
<span class="description"><p>Display label for the expression pedal overlay.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_analog_additionalProperties_max">
    <div class="card">
        <div class="card-header" id="headingpresets_items_analog_additionalProperties_max">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_analog_additionalProperties_max"
                        aria-expanded="" aria-controls="presets_items_analog_additionalProperties_max" onclick="setAnchor('#presets_items_analog_additionalProperties_max')"><span class="property-name">max</span></button>
            </h2>
        </div>

        <div id="presets_items_analog_additionalProperties_max"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_analog_additionalProperties_max"
             data-parent="#accordionpresets_items_analog_additionalProperties_max">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog" onclick="anchorLink('presets_items_analog')">analog</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog_additionalProperties" onclick="anchorLink('presets_items_analog_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog_additionalProperties_max" onclick="anchorLink('presets_items_analog_additionalProperties_max')">max</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Maximum CC value at toe position. Default: 127.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_analog_additionalProperties_min">
    <div class="card">
        <div class="card-header" id="headingpresets_items_analog_additionalProperties_min">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_analog_additionalProperties_min"
                        aria-expanded="" aria-controls="presets_items_analog_additionalProperties_min" onclick="setAnchor('#presets_items_analog_additionalProperties_min')"><span class="property-name">min</span></button>
            </h2>
        </div>

        <div id="presets_items_analog_additionalProperties_min"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_analog_additionalProperties_min"
             data-parent="#accordionpresets_items_analog_additionalProperties_min">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog" onclick="anchorLink('presets_items_analog')">analog</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog_additionalProperties" onclick="anchorLink('presets_items_analog_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_analog_additionalProperties_min" onclick="anchorLink('presets_items_analog_additionalProperties_min')">min</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Minimum CC value at heel position. Default: 0.</p>
</span>






            </div>
        </div>
    </div>
</div>
            </div>
        </div>
    </div>
</div>
            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons"
                        aria-expanded="" aria-controls="presets_items_buttons" onclick="setAnchor('#presets_items_buttons')"><span class="property-name">buttons</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons"
             data-parent="#accordionpresets_items_buttons">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Button configurations keyed by position: A, B, C, D, E, F.</p>
</span>






<div class="accordion" id="accordionpresets_items_buttons_additionalProperties">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties" onclick="setAnchor('#presets_items_buttons_additionalProperties')"><em><span class="property-name">Additional Properties</span></em></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties"
             data-parent="#accordionpresets_items_buttons_additionalProperties">
            <div class="card-body pl-5"><p class="additional-properties">Each additional property must conform to the following schema</p>

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">ButtonConfig</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Button configuration. Use one of: note, cc, program_change, or actions for the MIDI message.</p>
</span>








<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_actions">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_actions">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_actions"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_actions" onclick="setAnchor('#presets_items_buttons_additionalProperties_actions')"><span class="property-name">actions</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_actions"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_actions"
             data-parent="#accordionpresets_items_buttons_additionalProperties_actions">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a></div><span class="badge badge-dark value-type">Type: array or null</span><br/>
<span class="description"><p>Multi-action sequence: list of MIDI messages sent in order on press. Overrides cc/note/program_change fields.</p>
</span>





         <span class="badge badge-info no-additional">No Additional Items</span><h4>Each item of this array must be:</h4>
    <div class="card">
        <div class="card-body items-definition" id="presets_items_buttons_additionalProperties_actions_items">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">ActionYaml</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>A single action in a multi-action sequence. Exactly one type per entry.</p>
</span>

    <div class="any-of-value" id="presets_items_buttons_additionalProperties_actions_items_anyOf"><h2 class="handle">
  <label>Any of</label>
</h2><ul class="nav nav-tabs" id="tabspresets_items_buttons_additionalProperties_actions_items_anyOf_anyOf" role="tablist"><li class="nav-item">
            <a class="nav-link active anyOf-option"
               id="presets_items_buttons_additionalProperties_actions_items_anyOf_i0" data-toggle="tab" href="#tab-pane_presets_items_buttons_additionalProperties_actions_items_anyOf_i0" role="tab"
               onclick="setAnchor('#presets_items_buttons_additionalProperties_actions_items_anyOf_i0')"
            >Option 1</a>
        </li><li class="nav-item">
            <a class="nav-link anyOf-option"
               id="presets_items_buttons_additionalProperties_actions_items_anyOf_i1" data-toggle="tab" href="#tab-pane_presets_items_buttons_additionalProperties_actions_items_anyOf_i1" role="tab"
               onclick="setAnchor('#presets_items_buttons_additionalProperties_actions_items_anyOf_i1')"
            >Option 2</a>
        </li><li class="nav-item">
            <a class="nav-link anyOf-option"
               id="presets_items_buttons_additionalProperties_actions_items_anyOf_i2" data-toggle="tab" href="#tab-pane_presets_items_buttons_additionalProperties_actions_items_anyOf_i2" role="tab"
               onclick="setAnchor('#presets_items_buttons_additionalProperties_actions_items_anyOf_i2')"
            >Option 3</a>
        </li><li class="nav-item">
            <a class="nav-link anyOf-option"
               id="presets_items_buttons_additionalProperties_actions_items_anyOf_i3" data-toggle="tab" href="#tab-pane_presets_items_buttons_additionalProperties_actions_items_anyOf_i3" role="tab"
               onclick="setAnchor('#presets_items_buttons_additionalProperties_actions_items_anyOf_i3')"
            >Option 4</a>
        </li></ul>
<div class="tab-content card"><div class="tab-pane fade card-body active show"
             id="tab-pane_presets_items_buttons_additionalProperties_actions_items_anyOf_i0" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">actions items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i0" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i0')">item 0</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Wait between actions (milliseconds).</p>
</span>






<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay" onclick="setAnchor('#presets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay')"><span class="property-name">delay</span> <span class="badge badge-warning required-property">Required</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay"
             data-parent="#accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">actions items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i0" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay')">delay</a></div><span class="badge badge-dark value-type">Type: integer</span><span class="badge badge-info value-type">Format: uint16</span><br/>
<span class="description"><p>Delay in milliseconds before the next action.</p>
</span>



        <p><span class="badge badge-light restriction numeric-restriction" id="presets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay_number">Value must be greater or equal to <code>0.0</code></span></p>


            </div>
        </div>
    </div>
</div>
        </div><div class="tab-pane fade card-body "
             id="tab-pane_presets_items_buttons_additionalProperties_actions_items_anyOf_i1" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">actions items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i1" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i1')">item 1</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Send a Control Change message.</p>
</span>






<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc" onclick="setAnchor('#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc')"><span class="property-name">cc</span> <span class="badge badge-warning required-property">Required</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc"
             data-parent="#accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">actions items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i1" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i1')">item 1</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc')">cc</a></div><span class="badge badge-dark value-type">Type: integer</span><span class="badge badge-info value-type">Format: uint8</span><br/>
<span class="description"><p>CC number (0-127).</p>
</span>



        <p><span class="badge badge-light restriction numeric-restriction" id="presets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc_number">Value must be greater or equal to <code>0.0</code></span></p>


            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel" onclick="setAnchor('#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel')"><span class="property-name">channel</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel"
             data-parent="#accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">actions items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i1" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i1')">item 1</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel')">channel</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span><br/>
<span class="description"><p>MIDI channel (1-16). Inherits from button if omitted.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i1_value">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i1_value">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_value"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_actions_items_anyOf_i1_value" onclick="setAnchor('#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_value')"><span class="property-name">value</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_actions_items_anyOf_i1_value"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i1_value"
             data-parent="#accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i1_value">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">actions items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i1" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i1')">item 1</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_value" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i1_value')">value</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span><br/>
<span class="description"><p>CC value (0-127). Default: 127.</p>
</span>






            </div>
        </div>
    </div>
</div>
        </div><div class="tab-pane fade card-body "
             id="tab-pane_presets_items_buttons_additionalProperties_actions_items_anyOf_i2" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">actions items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i2" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i2')">item 2</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Send a Program Change message.</p>
</span>






<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel" onclick="setAnchor('#presets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel')"><span class="property-name">channel</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel"
             data-parent="#accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">actions items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i2" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i2')">item 2</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel')">channel</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span><br/>
<span class="description"><p>MIDI channel (1-16). Inherits from button if omitted.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change" onclick="setAnchor('#presets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change')"><span class="property-name">program_change</span> <span class="badge badge-warning required-property">Required</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change"
             data-parent="#accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">actions items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i2" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i2')">item 2</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change')">program_change</a></div><span class="badge badge-dark value-type">Type: integer</span><span class="badge badge-info value-type">Format: uint8</span><br/>
<span class="description"><p>Program number (0-127).</p>
</span>



        <p><span class="badge badge-light restriction numeric-restriction" id="presets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change_number">Value must be greater or equal to <code>0.0</code></span></p>


            </div>
        </div>
    </div>
</div>
        </div><div class="tab-pane fade card-body "
             id="tab-pane_presets_items_buttons_additionalProperties_actions_items_anyOf_i3" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">actions items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i3" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i3')">item 3</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Send a Note On message (velocity 127).</p>
</span>






<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel" onclick="setAnchor('#presets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel')"><span class="property-name">channel</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel"
             data-parent="#accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">actions items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i3" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i3')">item 3</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel')">channel</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span><br/>
<span class="description"><p>MIDI channel (1-16). Inherits from button if omitted.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i3_note">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i3_note">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_actions_items_anyOf_i3_note"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_actions_items_anyOf_i3_note" onclick="setAnchor('#presets_items_buttons_additionalProperties_actions_items_anyOf_i3_note')"><span class="property-name">note</span> <span class="badge badge-warning required-property">Required</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_actions_items_anyOf_i3_note"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_actions_items_anyOf_i3_note"
             data-parent="#accordionpresets_items_buttons_additionalProperties_actions_items_anyOf_i3_note">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions" onclick="anchorLink('presets_items_buttons_additionalProperties_actions')">actions</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')">actions items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i3" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i3')">item 3</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_actions_items_anyOf_i3_note" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items_anyOf_i3_note')">note</a></div><span class="badge badge-dark value-type">Type: integer</span><span class="badge badge-info value-type">Format: uint8</span><br/>
<span class="description"><p>MIDI note number (0-127).</p>
</span>



        <p><span class="badge badge-light restriction numeric-restriction" id="presets_items_buttons_additionalProperties_actions_items_anyOf_i3_note_number">Value must be greater or equal to <code>0.0</code></span></p>


            </div>
        </div>
    </div>
</div>
        </div></div></div>






        </div>
    </div>
            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_animation">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_animation">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_animation"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_animation" onclick="setAnchor('#presets_items_buttons_additionalProperties_animation')"><span class="property-name">animation</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_animation"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_animation"
             data-parent="#accordionpresets_items_buttons_additionalProperties_animation">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_animation" onclick="anchorLink('presets_items_buttons_additionalProperties_animation')">animation</a></div><span class="badge badge-dark value-type">Type: string or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>LED animation when active. Values: solid, blink, pulse, rotate, colorcycle.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_cc">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_cc">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_cc"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_cc" onclick="setAnchor('#presets_items_buttons_additionalProperties_cc')"><span class="property-name">cc</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_cc"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_cc"
             data-parent="#accordionpresets_items_buttons_additionalProperties_cc">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_cc" onclick="anchorLink('presets_items_buttons_additionalProperties_cc')">cc</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Send Control Change. Combined with toggle/values for different behaviors.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_channel">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_channel">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_channel"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_channel" onclick="setAnchor('#presets_items_buttons_additionalProperties_channel')"><span class="property-name">channel</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_channel"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_channel"
             data-parent="#accordionpresets_items_buttons_additionalProperties_channel">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_channel" onclick="anchorLink('presets_items_buttons_additionalProperties_channel')">channel</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>MIDI channel (1-16). Default: 1. Applies to all actions on this button unless overridden per-action.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_color">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_color">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_color"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_color" onclick="setAnchor('#presets_items_buttons_additionalProperties_color')"><span class="property-name">color</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_color"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_color"
             data-parent="#accordionpresets_items_buttons_additionalProperties_color">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_color" onclick="anchorLink('presets_items_buttons_additionalProperties_color')">color</a></div><span class="badge badge-dark value-type">Type: string or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>LED ring color when active. Values: red, green, blue, yellow, cyan, magenta, white, orange, purple, off, or #RRGGBB hex.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_label">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_label">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_label"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_label" onclick="setAnchor('#presets_items_buttons_additionalProperties_label')"><span class="property-name">label</span> <span class="badge badge-warning required-property">Required</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_label"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_label"
             data-parent="#accordionpresets_items_buttons_additionalProperties_label">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_label" onclick="anchorLink('presets_items_buttons_additionalProperties_label')">label</a></div><span class="badge badge-dark value-type">Type: string</span><br/>
<span class="description"><p>Display label shown on OLED (max 16 characters).</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_level">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_level">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_level"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_level" onclick="setAnchor('#presets_items_buttons_additionalProperties_level')"><span class="property-name">level</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_level"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_level"
             data-parent="#accordionpresets_items_buttons_additionalProperties_level">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_level" onclick="anchorLink('presets_items_buttons_additionalProperties_level')">level</a></div><span class="badge badge-dark value-type">Type: boolean or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Level mode: LED brightness reflects CC value (for multi-LED visualization).</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_note">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_note">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_note"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_note" onclick="setAnchor('#presets_items_buttons_additionalProperties_note')"><span class="property-name">note</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_note"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_note"
             data-parent="#accordionpresets_items_buttons_additionalProperties_note">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_note" onclick="anchorLink('presets_items_buttons_additionalProperties_note')">note</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Send Note On/Off. Button press = Note On (velocity 127), release = Note Off.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_on_long_press">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_on_long_press">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_on_long_press"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_on_long_press" onclick="setAnchor('#presets_items_buttons_additionalProperties_on_long_press')"><span class="property-name">on_long_press</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_on_long_press"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_on_long_press"
             data-parent="#accordionpresets_items_buttons_additionalProperties_on_long_press">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_on_long_press" onclick="anchorLink('presets_items_buttons_additionalProperties_on_long_press')">on_long_press</a></div><span class="badge badge-dark value-type">Type: string or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Action on long press (hold &gt; 500ms). Values: next<em>preset, prev</em>preset.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_program_change">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_program_change">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_program_change"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_program_change" onclick="setAnchor('#presets_items_buttons_additionalProperties_program_change')"><span class="property-name">program_change</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_program_change"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_program_change"
             data-parent="#accordionpresets_items_buttons_additionalProperties_program_change">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_program_change" onclick="anchorLink('presets_items_buttons_additionalProperties_program_change')">program_change</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Send Program Change on press.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_radio_group">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_radio_group">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_radio_group"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_radio_group" onclick="setAnchor('#presets_items_buttons_additionalProperties_radio_group')"><span class="property-name">radio_group</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_radio_group"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_radio_group"
             data-parent="#accordionpresets_items_buttons_additionalProperties_radio_group">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_radio_group" onclick="anchorLink('presets_items_buttons_additionalProperties_radio_group')">radio_group</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Radio group ID (0-255): only one button in the group can be active at a time. Pressing one deactivates others.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_renderer">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_renderer">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_renderer"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_renderer" onclick="setAnchor('#presets_items_buttons_additionalProperties_renderer')"><span class="property-name">renderer</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_renderer"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_renderer"
             data-parent="#accordionpresets_items_buttons_additionalProperties_renderer">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_renderer" onclick="anchorLink('presets_items_buttons_additionalProperties_renderer')">renderer</a></div><span class="badge badge-dark value-type">Type: string or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>LED spatial renderer. Values: solid (all 12), fill (partial arc), single (one LED), dots (evenly-spaced).</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_renderer_param">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_renderer_param">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_renderer_param"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_renderer_param" onclick="setAnchor('#presets_items_buttons_additionalProperties_renderer_param')"><span class="property-name">renderer_param</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_renderer_param"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_renderer_param"
             data-parent="#accordionpresets_items_buttons_additionalProperties_renderer_param">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_renderer_param" onclick="anchorLink('presets_items_buttons_additionalProperties_renderer_param')">renderer_param</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Renderer parameter: fill count (1-12), single position (0-11), or dot count (1-6).</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_reverse">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_reverse">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_reverse"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_reverse" onclick="setAnchor('#presets_items_buttons_additionalProperties_reverse')"><span class="property-name">reverse</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_reverse"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_reverse"
             data-parent="#accordionpresets_items_buttons_additionalProperties_reverse">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_reverse" onclick="anchorLink('presets_items_buttons_additionalProperties_reverse')">reverse</a></div><span class="badge badge-dark value-type">Type: boolean or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Reverse cycle direction (cycle values list goes backward).</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_toggle">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_toggle">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_toggle"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_toggle" onclick="setAnchor('#presets_items_buttons_additionalProperties_toggle')"><span class="property-name">toggle</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_toggle"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_toggle"
             data-parent="#accordionpresets_items_buttons_additionalProperties_toggle">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_toggle" onclick="anchorLink('presets_items_buttons_additionalProperties_toggle')">toggle</a></div><span class="badge badge-dark value-type">Type: boolean or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>Toggle mode: alternates between on<em>press (active) and on</em>release (inactive) on each press. LED stays lit while active.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_value">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_value">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_value"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_value" onclick="setAnchor('#presets_items_buttons_additionalProperties_value')"><span class="property-name">value</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_value"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_value"
             data-parent="#accordionpresets_items_buttons_additionalProperties_value">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_value" onclick="anchorLink('presets_items_buttons_additionalProperties_value')">value</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>CC value to send (default: 127). For toggle mode, this is the ON value.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_buttons_additionalProperties_values">
    <div class="card">
        <div class="card-header" id="headingpresets_items_buttons_additionalProperties_values">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_buttons_additionalProperties_values"
                        aria-expanded="" aria-controls="presets_items_buttons_additionalProperties_values" onclick="setAnchor('#presets_items_buttons_additionalProperties_values')"><span class="property-name">values</span></button>
            </h2>
        </div>

        <div id="presets_items_buttons_additionalProperties_values"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_buttons_additionalProperties_values"
             data-parent="#accordionpresets_items_buttons_additionalProperties_values">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_values" onclick="anchorLink('presets_items_buttons_additionalProperties_values')">values</a></div><span class="badge badge-dark value-type">Type: array of integer or null</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>CC cycle values: each press sends the next value in the list. Use with cc field.</p>
</span>





         <span class="badge badge-info no-additional">No Additional Items</span><h4>Each item of this array must be:</h4>
    <div class="card">
        <div class="card-body items-definition" id="presets_items_buttons_additionalProperties_values_items">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons" onclick="anchorLink('presets_items_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties" onclick="anchorLink('presets_items_buttons_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_values" onclick="anchorLink('presets_items_buttons_additionalProperties_values')">values</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_buttons_additionalProperties_values_items" onclick="anchorLink('presets_items_buttons_additionalProperties_values_items')">values items</a></div><span class="badge badge-dark value-type">Type: integer</span><span class="badge badge-info value-type">Format: uint8</span><br/>




        <p><span class="badge badge-light restriction numeric-restriction" id="presets_items_buttons_additionalProperties_values_items_number">Value must be greater or equal to <code>0.0</code></span></p>


        </div>
    </div>
            </div>
        </div>
    </div>
</div>
            </div>
        </div>
    </div>
</div>
            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_defaults">
    <div class="card">
        <div class="card-header" id="headingpresets_items_defaults">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_defaults"
                        aria-expanded="" aria-controls="presets_items_defaults" onclick="setAnchor('#presets_items_defaults')"><span class="property-name">defaults</span></button>
            </h2>
        </div>

        <div id="presets_items_defaults"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_defaults"
             data-parent="#accordionpresets_items_defaults">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults" onclick="anchorLink('presets_items_defaults')">defaults</a></div><br/>
<span class="description"><p>Initial state on first activation after upload. Determines which toggles start ON and encoder starting positions.</p>
</span><div class="any-of-value" id="presets_items_defaults_anyOf"><h2 class="handle">
  <label>Any of</label>
</h2><ul class="nav nav-tabs" id="tabspresets_items_defaults_anyOf_anyOf" role="tablist"><li class="nav-item">
            <a class="nav-link active anyOf-option"
               id="presets_items_defaults_anyOf_i0" data-toggle="tab" href="#tab-pane_presets_items_defaults_anyOf_i0" role="tab"
               onclick="setAnchor('#presets_items_defaults_anyOf_i0')"
            >DefaultsConfig</a>
        </li><li class="nav-item">
            <a class="nav-link anyOf-option"
               id="presets_items_defaults_anyOf_i1" data-toggle="tab" href="#tab-pane_presets_items_defaults_anyOf_i1" role="tab"
               onclick="setAnchor('#presets_items_defaults_anyOf_i1')"
            >Option 2</a>
        </li></ul>
<div class="tab-content card"><div class="tab-pane fade card-body active show"
             id="tab-pane_presets_items_defaults_anyOf_i0" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults" onclick="anchorLink('presets_items_defaults')">defaults</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf" onclick="anchorLink('presets_items_defaults_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf_i0" onclick="anchorLink('presets_items_defaults_anyOf_i0')">DefaultsConfig</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Default initial state for a preset on first activation after upload.</p>
</span>








<div class="accordion" id="accordionpresets_items_defaults_anyOf_i0_buttons">
    <div class="card">
        <div class="card-header" id="headingpresets_items_defaults_anyOf_i0_buttons">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_defaults_anyOf_i0_buttons"
                        aria-expanded="" aria-controls="presets_items_defaults_anyOf_i0_buttons" onclick="setAnchor('#presets_items_defaults_anyOf_i0_buttons')"><span class="property-name">buttons</span></button>
            </h2>
        </div>

        <div id="presets_items_defaults_anyOf_i0_buttons"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_defaults_anyOf_i0_buttons"
             data-parent="#accordionpresets_items_defaults_anyOf_i0_buttons">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults" onclick="anchorLink('presets_items_defaults')">defaults</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf" onclick="anchorLink('presets_items_defaults_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf_i0" onclick="anchorLink('presets_items_defaults_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf_i0_buttons" onclick="anchorLink('presets_items_defaults_anyOf_i0_buttons')">buttons</a></div><span class="badge badge-dark value-type">Type: object</span> <span class="badge badge-success default-value">Default: {}</span><br/>
<span class="description"><p>Button keys (A-F) mapped to "on" or "off". Omitted buttons default to off.</p>
</span>






<div class="accordion" id="accordionpresets_items_defaults_anyOf_i0_buttons_additionalProperties">
    <div class="card">
        <div class="card-header" id="headingpresets_items_defaults_anyOf_i0_buttons_additionalProperties">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_defaults_anyOf_i0_buttons_additionalProperties"
                        aria-expanded="" aria-controls="presets_items_defaults_anyOf_i0_buttons_additionalProperties" onclick="setAnchor('#presets_items_defaults_anyOf_i0_buttons_additionalProperties')"><em><span class="property-name">Additional Properties</span></em></button>
            </h2>
        </div>

        <div id="presets_items_defaults_anyOf_i0_buttons_additionalProperties"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_defaults_anyOf_i0_buttons_additionalProperties"
             data-parent="#accordionpresets_items_defaults_anyOf_i0_buttons_additionalProperties">
            <div class="card-body pl-5"><p class="additional-properties">Each additional property must conform to the following schema</p>

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults" onclick="anchorLink('presets_items_defaults')">defaults</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf" onclick="anchorLink('presets_items_defaults_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf_i0" onclick="anchorLink('presets_items_defaults_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf_i0_buttons" onclick="anchorLink('presets_items_defaults_anyOf_i0_buttons')">buttons</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf_i0_buttons_additionalProperties" onclick="anchorLink('presets_items_defaults_anyOf_i0_buttons_additionalProperties')">additionalProperties</a></div><span class="badge badge-dark value-type">Type: string</span><br/>







            </div>
        </div>
    </div>
</div>
            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_defaults_anyOf_i0_encoders">
    <div class="card">
        <div class="card-header" id="headingpresets_items_defaults_anyOf_i0_encoders">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_defaults_anyOf_i0_encoders"
                        aria-expanded="" aria-controls="presets_items_defaults_anyOf_i0_encoders" onclick="setAnchor('#presets_items_defaults_anyOf_i0_encoders')"><span class="property-name">encoders</span></button>
            </h2>
        </div>

        <div id="presets_items_defaults_anyOf_i0_encoders"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_defaults_anyOf_i0_encoders"
             data-parent="#accordionpresets_items_defaults_anyOf_i0_encoders">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults" onclick="anchorLink('presets_items_defaults')">defaults</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf" onclick="anchorLink('presets_items_defaults_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf_i0" onclick="anchorLink('presets_items_defaults_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf_i0_encoders" onclick="anchorLink('presets_items_defaults_anyOf_i0_encoders')">encoders</a></div><span class="badge badge-dark value-type">Type: object</span> <span class="badge badge-success default-value">Default: {}</span><br/>
<span class="description"><p>Encoder keys (Vol, Gain) mapped to initial value (0-127). Omitted encoders default to 0.</p>
</span>






<div class="accordion" id="accordionpresets_items_defaults_anyOf_i0_encoders_additionalProperties">
    <div class="card">
        <div class="card-header" id="headingpresets_items_defaults_anyOf_i0_encoders_additionalProperties">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_defaults_anyOf_i0_encoders_additionalProperties"
                        aria-expanded="" aria-controls="presets_items_defaults_anyOf_i0_encoders_additionalProperties" onclick="setAnchor('#presets_items_defaults_anyOf_i0_encoders_additionalProperties')"><em><span class="property-name">Additional Properties</span></em></button>
            </h2>
        </div>

        <div id="presets_items_defaults_anyOf_i0_encoders_additionalProperties"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_defaults_anyOf_i0_encoders_additionalProperties"
             data-parent="#accordionpresets_items_defaults_anyOf_i0_encoders_additionalProperties">
            <div class="card-body pl-5"><p class="additional-properties">Each additional property must conform to the following schema</p>

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults" onclick="anchorLink('presets_items_defaults')">defaults</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf" onclick="anchorLink('presets_items_defaults_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf_i0" onclick="anchorLink('presets_items_defaults_anyOf_i0')">item 0</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf_i0_encoders" onclick="anchorLink('presets_items_defaults_anyOf_i0_encoders')">encoders</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf_i0_encoders_additionalProperties" onclick="anchorLink('presets_items_defaults_anyOf_i0_encoders_additionalProperties')">additionalProperties</a></div><span class="badge badge-dark value-type">Type: integer</span><span class="badge badge-info value-type">Format: uint8</span><br/>




        <p><span class="badge badge-light restriction numeric-restriction" id="presets_items_defaults_anyOf_i0_encoders_additionalProperties_number">Value must be greater or equal to <code>0.0</code></span></p>


            </div>
        </div>
    </div>
</div>
            </div>
        </div>
    </div>
</div>
        </div><div class="tab-pane fade card-body "
             id="tab-pane_presets_items_defaults_anyOf_i1" role="tabpanel">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults" onclick="anchorLink('presets_items_defaults')">defaults</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf" onclick="anchorLink('presets_items_defaults_anyOf')">anyOf</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_defaults_anyOf_i1" onclick="anchorLink('presets_items_defaults_anyOf_i1')">item 1</a></div><span class="badge badge-dark value-type">Type: null</span><br/>







        </div></div></div>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_encoders">
    <div class="card">
        <div class="card-header" id="headingpresets_items_encoders">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_encoders"
                        aria-expanded="" aria-controls="presets_items_encoders" onclick="setAnchor('#presets_items_encoders')"><span class="property-name">encoders</span></button>
            </h2>
        </div>

        <div id="presets_items_encoders"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_encoders"
             data-parent="#accordionpresets_items_encoders">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_encoders" onclick="anchorLink('presets_items_encoders')">encoders</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Encoder configurations keyed by position: Vol (left), Gain (right).</p>
</span>






<div class="accordion" id="accordionpresets_items_encoders_additionalProperties">
    <div class="card">
        <div class="card-header" id="headingpresets_items_encoders_additionalProperties">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_encoders_additionalProperties"
                        aria-expanded="" aria-controls="presets_items_encoders_additionalProperties" onclick="setAnchor('#presets_items_encoders_additionalProperties')"><em><span class="property-name">Additional Properties</span></em></button>
            </h2>
        </div>

        <div id="presets_items_encoders_additionalProperties"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_encoders_additionalProperties"
             data-parent="#accordionpresets_items_encoders_additionalProperties">
            <div class="card-body pl-5"><p class="additional-properties">Each additional property must conform to the following schema</p>

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_encoders" onclick="anchorLink('presets_items_encoders')">encoders</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_encoders_additionalProperties" onclick="anchorLink('presets_items_encoders_additionalProperties')">EncoderConfig</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>Rotary encoder configuration.</p>
</span>








<div class="accordion" id="accordionpresets_items_encoders_additionalProperties_cc">
    <div class="card">
        <div class="card-header" id="headingpresets_items_encoders_additionalProperties_cc">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_encoders_additionalProperties_cc"
                        aria-expanded="" aria-controls="presets_items_encoders_additionalProperties_cc" onclick="setAnchor('#presets_items_encoders_additionalProperties_cc')"><span class="property-name">cc</span></button>
            </h2>
        </div>

        <div id="presets_items_encoders_additionalProperties_cc"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_encoders_additionalProperties_cc"
             data-parent="#accordionpresets_items_encoders_additionalProperties_cc">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_encoders" onclick="anchorLink('presets_items_encoders')">encoders</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_encoders_additionalProperties" onclick="anchorLink('presets_items_encoders_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_encoders_additionalProperties_cc" onclick="anchorLink('presets_items_encoders_additionalProperties_cc')">cc</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint16</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>MIDI CC number to send (0-127). Each detent click increments/decrements the value.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_encoders_additionalProperties_channel">
    <div class="card">
        <div class="card-header" id="headingpresets_items_encoders_additionalProperties_channel">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_encoders_additionalProperties_channel"
                        aria-expanded="" aria-controls="presets_items_encoders_additionalProperties_channel" onclick="setAnchor('#presets_items_encoders_additionalProperties_channel')"><span class="property-name">channel</span></button>
            </h2>
        </div>

        <div id="presets_items_encoders_additionalProperties_channel"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_encoders_additionalProperties_channel"
             data-parent="#accordionpresets_items_encoders_additionalProperties_channel">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_encoders" onclick="anchorLink('presets_items_encoders')">encoders</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_encoders_additionalProperties" onclick="anchorLink('presets_items_encoders_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_encoders_additionalProperties_channel" onclick="anchorLink('presets_items_encoders_additionalProperties_channel')">channel</a></div><span class="badge badge-dark value-type">Type: integer or null</span><span class="badge badge-info value-type">Format: uint8</span> <span class="badge badge-success default-value">Default: null</span><br/>
<span class="description"><p>MIDI channel (1-16). Default: 1.</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_encoders_additionalProperties_label">
    <div class="card">
        <div class="card-header" id="headingpresets_items_encoders_additionalProperties_label">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_encoders_additionalProperties_label"
                        aria-expanded="" aria-controls="presets_items_encoders_additionalProperties_label" onclick="setAnchor('#presets_items_encoders_additionalProperties_label')"><span class="property-name">label</span> <span class="badge badge-warning required-property">Required</span></button>
            </h2>
        </div>

        <div id="presets_items_encoders_additionalProperties_label"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_encoders_additionalProperties_label"
             data-parent="#accordionpresets_items_encoders_additionalProperties_label">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_encoders" onclick="anchorLink('presets_items_encoders')">encoders</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_encoders_additionalProperties" onclick="anchorLink('presets_items_encoders_additionalProperties')">additionalProperties</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_encoders_additionalProperties_label" onclick="anchorLink('presets_items_encoders_additionalProperties_label')">label</a></div><span class="badge badge-dark value-type">Type: string</span><br/>
<span class="description"><p>Display label shown on OLED overlay when turning.</p>
</span>






            </div>
        </div>
    </div>
</div>
            </div>
        </div>
    </div>
</div>
            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_name">
    <div class="card">
        <div class="card-header" id="headingpresets_items_name">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_name"
                        aria-expanded="" aria-controls="presets_items_name" onclick="setAnchor('#presets_items_name')"><span class="property-name">name</span> <span class="badge badge-warning required-property">Required</span></button>
            </h2>
        </div>

        <div id="presets_items_name"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_name"
             data-parent="#accordionpresets_items_name">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_name" onclick="anchorLink('presets_items_name')">name</a></div><span class="badge badge-dark value-type">Type: string</span><br/>
<span class="description"><p>Preset name displayed on the OLED (max 16 characters).</p>
</span>






            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_on_enter">
    <div class="card">
        <div class="card-header" id="headingpresets_items_on_enter">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_on_enter"
                        aria-expanded="" aria-controls="presets_items_on_enter" onclick="setAnchor('#presets_items_on_enter')"><span class="property-name">on_enter</span></button>
            </h2>
        </div>

        <div id="presets_items_on_enter"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_on_enter"
             data-parent="#accordionpresets_items_on_enter">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_on_enter" onclick="anchorLink('presets_items_on_enter')">on_enter</a></div><span class="badge badge-dark value-type">Type: array or null</span><br/>
<span class="description"><p>MIDI messages sent automatically when this preset becomes active (on switch or boot).</p>
</span>





         <span class="badge badge-info no-additional">No Additional Items</span><h4>Each item of this array must be:</h4>
    <div class="card">
        <div class="card-body items-definition" id="presets_items_on_enter_items">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_on_enter" onclick="anchorLink('presets_items_on_enter')">on_enter</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_on_enter_items" onclick="anchorLink('presets_items_on_enter_items')">ActionYaml</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>A single action in a multi-action sequence. Exactly one type per entry.</p>
</span><a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')" class="ref-link">Same definition as presets_items_buttons_additionalProperties_actions_items</a>
        </div>
    </div>
            </div>
        </div>
    </div>
</div>
<div class="accordion" id="accordionpresets_items_on_exit">
    <div class="card">
        <div class="card-header" id="headingpresets_items_on_exit">
            <h2 class="mb-0">
                <button class="btn btn-link property-name-button" type="button" data-toggle="collapse" data-target="#presets_items_on_exit"
                        aria-expanded="" aria-controls="presets_items_on_exit" onclick="setAnchor('#presets_items_on_exit')"><span class="property-name">on_exit</span></button>
            </h2>
        </div>

        <div id="presets_items_on_exit"
             class="collapse property-definition-div" aria-labelledby="headingpresets_items_on_exit"
             data-parent="#accordionpresets_items_on_exit">
            <div class="card-body pl-5">

    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_on_exit" onclick="anchorLink('presets_items_on_exit')">on_exit</a></div><span class="badge badge-dark value-type">Type: array or null</span><br/>
<span class="description"><p>MIDI messages sent automatically when leaving this preset.</p>
</span>





         <span class="badge badge-info no-additional">No Additional Items</span><h4>Each item of this array must be:</h4>
    <div class="card">
        <div class="card-body items-definition" id="presets_items_on_exit_items">


    <div class="breadcrumbs">root
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets" onclick="anchorLink('presets')">presets</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items" onclick="anchorLink('presets_items')">presets items</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_on_exit" onclick="anchorLink('presets_items_on_exit')">on_exit</a>
        <svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-arrow-right-short" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path
                fill-rule="evenodd"
                d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8z"
            />
        </svg>
    <a href="#presets_items_on_exit_items" onclick="anchorLink('presets_items_on_exit_items')">ActionYaml</a></div><span class="badge badge-dark value-type">Type: object</span><br/>
<span class="description"><p>A single action in a multi-action sequence. Exactly one type per entry.</p>
</span><a href="#presets_items_buttons_additionalProperties_actions_items" onclick="anchorLink('presets_items_buttons_additionalProperties_actions_items')" class="ref-link">Same definition as presets_items_buttons_additionalProperties_actions_items</a>
        </div>
    </div>
            </div>
        </div>
    </div>
</div>
        </div>
    </div>
            </div>
        </div>
    </div>
</div>

    <footer>
        <p class="generated-by-footer">Generated using <a href="https://github.com/coveooss/json-schema-for-humans">json-schema-for-humans</a> on 2026-07-02 at 08:27:57 +0200</p>
    </footer></body>
</html>
