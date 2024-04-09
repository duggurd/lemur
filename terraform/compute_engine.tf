resource "google_compute_instance" "mitm" {
  boot_disk {
    auto_delete = true
    device_name = "mitm"

    initialize_params {
      image = "projects/debian-cloud/global/images/debian-12-bookworm-v20240312"
      size  = 10
      type  = "pd-balanced"
    }

    mode = "READ_WRITE"
  }

  can_ip_forward      = false
  deletion_protection = false
  enable_display      = false

  labels = {
    goog-ec-src = "vm_add-tf"
  }

  machine_type = "e2-micro"
  name         = "mitm"

  network_interface {
    access_config {
      network_tier = "STANDARD"
    }


    queue_count = 0
    stack_type  = "IPV4_ONLY"
    subnetwork  = "projects/lemur-419709/regions/europe-north1/subnetworks/default"
  }

  scheduling {
    automatic_restart   = false
    on_host_maintenance = "TERMINATE"
    provisioning_model  = "SPOT"
    preemptible = true
  }

  service_account {
    email  = "lemur-7@lemur-419709.iam.gserviceaccount.com"
    scopes = ["https://www.googleapis.com/auth/devstorage.read_only", "https://www.googleapis.com/auth/logging.write", "https://www.googleapis.com/auth/monitoring.write", "https://www.googleapis.com/auth/service.management.readonly", "https://www.googleapis.com/auth/servicecontrol", "https://www.googleapis.com/auth/trace.append"]
  }

  shielded_instance_config {
    enable_integrity_monitoring = true
    enable_secure_boot          = false
    enable_vtpm                 = true
  }

  tags = ["http-server", "https-server"]
  zone = "europe-north1-a"
}


resource "google_" "name" {
  
}