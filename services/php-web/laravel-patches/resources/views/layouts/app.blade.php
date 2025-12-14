<!doctype html>
<html lang="ru">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Space Dashboard</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"/>
    <style>
        :root {
            --bg-primary: #0a0a0a; 
            --bg-secondary: #1a1a1a;
            --text-color: #f0f0f0; 
            --accent-color: #4a90e2;
            --border-color: #333333;
        }

        body {
            background-color: var(--bg-primary);
            color: var(--text-color);
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
        }

        .navbar {
            background-color: var(--bg-secondary);
            border-bottom: 1px solid var(--border-color);
            box-shadow: none;
            padding: 1rem 0;
        }
        .navbar-brand {
            color: var(--accent-color) !important;
            font-weight: 700;
        }
        .nav-link {
            color: var(--text-color) !important;
            transition: color 0.2s;
        }
        .nav-link:hover {
            color: var(--accent-color) !important;
            opacity: 0.8;
        }

        /* минимализм чут-чут */
        .card { 
            background-color: var(--bg-secondary);
            border: 1px solid var(--border-color);
            transition: border-color 0.2s;
            box-shadow: none;
            border-radius: 4px;
        }
        .card:hover { 
            transform: none;
            border-color: var(--accent-color);
        }

        /* для вайба */
        #map{height:340px; border: 1px solid var(--border-color);}
        .form-control {
            background-color: #1a1a1a;
            border-color: var(--border-color);
            color: var(--text-color);
        }
        .btn-outline-light {
            color: var(--accent-color);
            border-color: var(--accent-color);
        }
        .btn-outline-light:hover {
            background-color: var(--accent-color);
            color: var(--bg-secondary) !important;
        }

        .fade-in { animation: fadeIn 0.4s ease-in-out; }
        @keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }
    </style>
    <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
</head>
<body>
<nav class="navbar navbar-expand-lg mb-4">
    <div class="container">
        <a class="navbar-brand" href="/dashboard">КАССИОПЕЯ</a>
        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav">
            <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarNav">
            <ul class="navbar-nav me-auto">
                <li class="nav-item"><a class="nav-link" href="/dashboard">Dashboard</a></li>
                <li class="nav-item"><a class="nav-link" href="/osdr">OSDR Data</a></li>
                <li class="nav-item"><a class="nav-link" href="/legacy">Legacy Telemetry</a></li>
            </ul>
            <form class="d-flex" action="/search" method="GET">
                <input class="form-control me-2" type="search" name="q" placeholder="Search data..." aria-label="Search">
                <button class="btn btn-outline-light" type="submit">Search</button>
            </form>
        </div>
    </div>
</nav>
<div class="container fade-in">
    @yield('content')
</div>
<script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
</body>
</html>