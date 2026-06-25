using Microsoft.EntityFrameworkCore;
using BSAP.Common.Models;

namespace BSAP.Orchestration;

public class BSAPDbContext : DbContext
{
    public DbSet<ScanRecord> Scans { get; set; } = null!;

    public BSAPDbContext(DbContextOptions<BSAPDbContext> options) : base(options) { }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<ScanRecord>().HasKey(s => s.Id);
    }
}

public class ScanRecord
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string ScanId { get; set; } = string.Empty;
    public string Type { get; set; } = string.Empty;
    public DateTime StartedAt { get; set; } = DateTime.UtcNow;
    public DateTime? CompletedAt { get; set; }
    public string Status { get; set; } = "Pending";
    public string ResultJson { get; set; } = string.Empty;
}