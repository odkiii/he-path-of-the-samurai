<!doctype html>
<html lang="ru">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Space Dashboard</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"/>
    <style>
        /*
        ========================================
        LIGHT MINIMALIST THEME
        ========================================
        */
        :root {
            --bg-primary: #f8f9fa; /* Светлый фон (почти белый) */
            --bg-secondary: #ffffff; /* Белый фон для карточек */
            --text-color: #212529; /* Темный текст */
            --accent-color: #007bff; /* Чистый синий акцент */
            --border-color: #e9ecef; /* Очень тонкая светло-серая линия */
            --shadow-subtle: 0 1px 3px rgba(0, 0, 0, 0.05); /* Легкая тень для "парения" */
        }

        /* 1. Общий Стиль и Фон */
        body {
            background-color: var(--bg-primary);
            color: var(--text-color);
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            transition: background-color 0.3s;
        }

        /* 2. Навигация (Чистота и Контраст) */
        .navbar {
            background-color: var(--bg-secondary);
            border-bottom: 1px solid var(--border-color); 
            box-shadow: var(--shadow-subtle);
            padding: 1rem 0;
        }
        .navbar-brand {
            color: var(--accent-color) !important;
            font-weight: 700;
        }
        .nav-link {
            color: var(--text-color) !important;
            font-weight: 500;
            border-bottom: 3px solid transparent; /* Для эффекта активной ссылки */
            transition: color 0.2s, border-bottom-color 0.2s;
        }
        .nav-link:hover {
            color: var(--accent-color) !important;
            border-bottom-color: var(--accent-color);
            opacity: 1;
        }
        .nav-item .active {
            border-bottom-color: var(--accent-color);
        }

        /* 3. Карточки (Плоский дизайн) */
        .card { 
            background-color: var(--bg-secondary);
            border: 1px solid var(--border-color);
            transition: box-shadow 0.2s, border-color 0.2s;
            box-shadow: var(--shadow-subtle);
            border-radius: 6px;
        }
        .card:hover { 
            /* Усиленный эффект при наведении, но без сдвига */
            box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1); 
            border-color: var(--accent-color);
        }
        .card-header {
            background-color: var(--bg-primary); /* Легкий контраст */
            border-bottom: 1px solid var(--border-color);
            font-weight: 600;
        }

        /* 4. Формы и Интерактивность */
        #map{
            height:340px; 
            border: 1px solid var(--border-color); 
            border-radius: 6px;
        }
        .form-control {
            background-color: #ffffff;
            border-color: #ced4da;
            color: var(--text-color);
            transition: border-color 0.2s, box-shadow 0.2s;
        }
        .form-control:focus {
            border-color: var(--accent-color);
            box-shadow: 0 0 0 0.25rem rgba(0, 123, 255, 0.25); /* Акцентное свечение */
        }
        
        .btn-outline-light {
            color: var(--accent-color);
            border-color: var(--accent-color);
            /* Переопределяем Bootstrap-класс, который тут уже не Light */
            background-color: transparent;
        }
        .btn-outline-light:hover {
            background-color: var(--accent-color);
            color: #ffffff !important;
            border-color: var(--accent-color);
        }

        /* Анимация */
        .fade-in { animation: fadeIn 0.4s ease-in-out; }
        @keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }
    </style>
    <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
</head>
<body>
<nav class="navbar navbar-expand-lg mb-4">
    <div class="container">
        <a class="navbar-brand" href="/dashboard">✨ КАССИОПЕЯ</a>
        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav">
            <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarNav">
            <ul class="navbar-nav me-auto">
                <li class="nav-item"><a class="nav-link active" aria-current="page" href="/dashboard">Dashboard</a></li>
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