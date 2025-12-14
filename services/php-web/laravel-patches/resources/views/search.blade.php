@extends('layouts.app')

@section('content')
<div class="row">
    <div class="col-12">
        <h2 class="mb-4">Search Results for "{{ $query }}"</h2>
        
        <div class="card p-3 mb-4">
            <form action="/search" method="GET" class="row g-3">
                <div class="col-md-8">
                    <input type="text" name="q" class="form-control" value="{{ $query }}" placeholder="Keywords...">
                </div>
                <div class="col-md-3">
                    <select name="filter" class="form-select">
                        <option value="all" {{ $filter == 'all' ? 'selected' : '' }}>All Sources</option>
                        <option value="osdr" {{ $filter == 'osdr' ? 'selected' : '' }}>OSDR</option>
                        <option value="iss" {{ $filter == 'iss' ? 'selected' : '' }}>ISS Logs</option>
                    </select>
                </div>
                <div class="col-md-1">
                    <button type="submit" class="btn btn-primary w-100">Go</button>
                </div>
            </form>
        </div>

        @if(count($results) > 0)
            <div class="list-group">
                @foreach($results as $item)
                    <a href="{{ $item['link'] }}" class="list-group-item list-group-item-action">
                        <div class="d-flex w-100 justify-content-between">
                            <h5 class="mb-1">{{ $item['title'] }}</h5>
                            <small>{{ $item['type'] }}</small>
                        </div>
                    </a>
                @endforeach
            </div>
        @else
            <div class="alert alert-info">No results found.</div>
        @endif
    </div>
</div>
@endsection