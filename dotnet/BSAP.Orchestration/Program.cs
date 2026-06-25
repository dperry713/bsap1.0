using BSAP.Orchestration.RiskEngine;
using BSAP.Orchestration.Services;
using Grpc.Net.Client;
using Hangfire;
using Hangfire.PostgreSql;
using Microsoft.EntityFrameworkCore;
using Microsoft.OpenApi.Models;

var builder = WebApplication.CreateBuilder(args);

// Add services
builder.Services.AddControllers();
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen(c =>
{
    c.SwaggerDoc("v1", new OpenApiInfo { Title = "BSAP Orchestration API", Version = "v1" });
});

// gRPC Client to Rust Scanner
var rustChannel = GrpcChannel.ForAddress(builder.Configuration["RustScanner:Url"] ?? "http://localhost:50051");
builder.Services.AddSingleton(rustChannel);
builder.Services.AddSingleton<IScannerClient, GrpcScannerClient>();

// Risk Engine
builder.Services.AddSingleton<RiskEngine>();

// Hangfire for background jobs
builder.Services.AddHangfire(config =>
{
    config.SetDataCompatibilityLevel(CompatibilityLevel.Version_180)
          .UseSimpleAssemblyNameTypeSerializer()
          .UseRecommendedSerializerSettings()
          .UsePostgreSqlStorage(builder.Configuration.GetConnectionString("DefaultConnection"));
});
builder.Services.AddHangfireServer();

// EF Core (for scan history)
builder.Services.AddDbContext<BSAPDbContext>(options =>
    options.UseNpgsql(builder.Configuration.GetConnectionString("DefaultConnection")));

// MediatR for CQRS
builder.Services.AddMediatR(cfg => cfg.RegisterServicesFromAssembly(typeof(Program).Assembly));

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();
app.UseAuthorization();
app.UseHangfireDashboard("/hangfire");

app.MapControllers();

app.Run();