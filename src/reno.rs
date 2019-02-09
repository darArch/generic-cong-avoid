extern crate slog;

use portus::DatapathInfo;
use std::net::Ipv4Addr;
use GenericCongAvoidAlg;
use GenericCongAvoidFlow;
use GenericCongAvoidMeasurements;

#[derive(Default)]
pub struct Reno {
    logger: Option<slog::Logger>,
    mss: u32,
    init_cwnd: f64,
    cwnd: f64,
    src_ip: Option<String>,
    src_port: Option<u32>,
    dst_ip: Option<String>,
    dst_port: Option<u32>,
}

impl GenericCongAvoidAlg for Reno {
    type Flow = Self;

    fn name() -> &'static str {
        eprintln!("{}: reno/name", system_time());
        "reno"
    }

    fn with_args(_: clap::ArgMatches) -> Self {
        eprintln!("{}: reno/with_args", system_time());
        Default::default()
    }

    fn new_flow(&self, logger: Option<slog::Logger>, info: &DatapathInfo) -> Self::Flow {
        eprintln!("{}: reno/new_flow", system_time());
        let le_src_ip = info.src_ip.to_be();
        let src_ip = Ipv4Addr::from(le_src_ip);
        let le_dst_ip = info.dst_ip.to_be();
        let dst_ip = Ipv4Addr::from(le_dst_ip);

        Reno {
            logger,
            mss: info.mss,
            init_cwnd: f64::from(info.init_cwnd),
            cwnd: f64::from(info.init_cwnd),
            src_ip: Some(src_ip.to_string()),
            src_port: Some(info.src_port),
            dst_ip: Some(dst_ip.to_string()),
            dst_port: Some(info.dst_port),
        }
    }
}

impl GenericCongAvoidFlow for Reno {
    fn curr_cwnd(&self) -> u32 {
        eprintln!("{}: reno/curr_cwnd", system_time());
        self.logger.as_ref().map(|log| {
            info!(log, "curr_cwnd()";
                "curr_cwnd (bytes)" => self.cwnd,
                "mss (bytes)" => self.mss,
                "src_ip" => self.src_ip.as_ref(),
                "src_port" => self.src_port,
                "dst_ip" => self.dst_ip.as_ref(),
                "dst_port" => self.dst_port,
            );
        });

        self.cwnd as u32
    }

    fn set_cwnd(&mut self, cwnd: u32) {
        eprintln!("{}: reno/set_cwnd", system_time());
        self.cwnd = f64::from(cwnd);

        self.logger.as_ref().map(|log| {
            info!(log, "set_cwnd()";
                "curr_cwnd (bytes)" => self.cwnd,
                "mss (bytes)" => self.mss,
                "src_ip" => self.src_ip.as_ref(),
                "src_port" => self.src_port,
                "dst_ip" => self.dst_ip.as_ref(),
                "dst_port" => self.dst_port,
            );
        });
    }

    fn increase(&mut self, m: &GenericCongAvoidMeasurements) {
        eprintln!("{}: reno/increase", system_time());
        // increase cwnd by 1 / cwnd per packet
        self.cwnd += f64::from(self.mss) * (f64::from(m.acked) / self.cwnd);

        self.logger.as_ref().map(|log| {
            info!(log, "increase()";
                "curr_cwnd (bytes)" => self.cwnd,
                "mss (bytes)" => self.mss,
                "src_ip" => self.src_ip.as_ref(),
                "src_port" => self.src_port,
                "dst_ip" => self.dst_ip.as_ref(),
                "dst_port" => self.dst_port,
            );
        });
    }

    fn reduction(&mut self, _m: &GenericCongAvoidMeasurements) {
        eprintln!("{}: reno/reduction", system_time());
        self.cwnd /= 2.0;
        if self.cwnd <= self.init_cwnd {
            self.cwnd = self.init_cwnd;
        }

        self.logger.as_ref().map(|log| {
            info!(log, "reduction()";
                "curr_cwnd (bytes)" => self.cwnd,
                "mss (bytes)" => self.mss,
                "src_ip" => self.src_ip.as_ref(),
                "src_port" => self.src_port,
                "dst_ip" => self.dst_ip.as_ref(),
                "dst_port" => self.dst_port,
            );
        });
    }
}

fn system_time() -> u128 {
    let now = std::time::SystemTime::now();
    let ts = now.duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");
    ts.as_millis()
}