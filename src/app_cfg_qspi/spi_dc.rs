#[doc = "Register `SPI_DC` reader"]
pub type R = crate::R<SpiDcSpec>;
#[doc = "Register `SPI_DC` writer"]
pub type W = crate::W<SpiDcSpec>;
#[doc = "Field `CKP0` reader - 0:0\\]
Clock polarity for chip select 0 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
pub type Ckp0R = crate::BitReader;
#[doc = "Field `CKP0` writer - 0:0\\]
Clock polarity for chip select 0 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
pub type Ckp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSP0` reader - 1:1\\]
Chip select polarity for chip select 0 0- Active low 1- Active high"]
pub type Csp0R = crate::BitReader;
#[doc = "Field `CSP0` writer - 1:1\\]
Chip select polarity for chip select 0 0- Active low 1- Active high"]
pub type Csp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPH0` reader - 2:2\\]
Clock phase for chip select 0. If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
pub type Ckph0R = crate::BitReader;
#[doc = "Field `CKPH0` writer - 2:2\\]
Clock phase for chip select 0. If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
pub type Ckph0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DD0` reader - 4:3\\]
Data delay for chip select 0 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
pub type Dd0R = crate::FieldReader;
#[doc = "Field `DD0` writer - 4:3\\]
Data delay for chip select 0 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
pub type Dd0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved1` reader - 7:5\\]
Always read as 0"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 7:5\\]
Always read as 0"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKP1` reader - 8:8\\]
Clock polarity for chip select 1 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
pub type Ckp1R = crate::BitReader;
#[doc = "Field `CKP1` writer - 8:8\\]
Clock polarity for chip select 1 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
pub type Ckp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSP1` reader - 9:9\\]
Chip select polarity for chip select 1 0- Active low 1- Active high"]
pub type Csp1R = crate::BitReader;
#[doc = "Field `CSP1` writer - 9:9\\]
Chip select polarity for chip select 1 0- Active low 1- Active high"]
pub type Csp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPH1` reader - 10:10\\]
Clock phase for chip select 1. If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
pub type Ckph1R = crate::BitReader;
#[doc = "Field `CKPH1` writer - 10:10\\]
Clock phase for chip select 1. If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
pub type Ckph1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DD1` reader - 12:11\\]
Data delay for chip select 1 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
pub type Dd1R = crate::FieldReader;
#[doc = "Field `DD1` writer - 12:11\\]
Data delay for chip select 1 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
pub type Dd1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - 15:13\\]
Always read as 0"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 15:13\\]
Always read as 0"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKP2` reader - 16:16\\]
Clock polarity for chip select 2 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
pub type Ckp2R = crate::BitReader;
#[doc = "Field `CKP2` writer - 16:16\\]
Clock polarity for chip select 2 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
pub type Ckp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSP2` reader - 17:17\\]
Chip select polarity for chip select 2 0- Active low 1- Active high"]
pub type Csp2R = crate::BitReader;
#[doc = "Field `CSP2` writer - 17:17\\]
Chip select polarity for chip select 2 0- Active low 1- Active high"]
pub type Csp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPH2` reader - 18:18\\]
Clock phase for chip select 2. If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
pub type Ckph2R = crate::BitReader;
#[doc = "Field `CKPH2` writer - 18:18\\]
Clock phase for chip select 2. If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
pub type Ckph2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DD2` reader - 20:19\\]
Data delay for chip select 2 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
pub type Dd2R = crate::FieldReader;
#[doc = "Field `DD2` writer - 20:19\\]
Data delay for chip select 2 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
pub type Dd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - 23:21\\]
Always read as 0"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - 23:21\\]
Always read as 0"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKP3` reader - 24:24\\]
Clock polarity for chip select 3 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
pub type Ckp3R = crate::BitReader;
#[doc = "Field `CKP3` writer - 24:24\\]
Clock polarity for chip select 3 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
pub type Ckp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSP3` reader - 25:25\\]
Chip select polarity for chip select 3 0- Active low 1- Active high"]
pub type Csp3R = crate::BitReader;
#[doc = "Field `CSP3` writer - 25:25\\]
Chip select polarity for chip select 3 0- Active low 1- Active high"]
pub type Csp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPH3` reader - 26:26\\]
Clock phase for chip select 3 If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
pub type Ckph3R = crate::BitReader;
#[doc = "Field `CKPH3` writer - 26:26\\]
Clock phase for chip select 3 If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
pub type Ckph3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DD3` reader - 28:27\\]
Data delay for chip select 3 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
pub type Dd3R = crate::FieldReader;
#[doc = "Field `DD3` writer - 28:27\\]
Data delay for chip select 3 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
pub type Dd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved4` reader - 31:29\\]
Always read as 0"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - 31:29\\]
Always read as 0"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clock polarity for chip select 0 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
    #[inline(always)]
    pub fn ckp0(&self) -> Ckp0R {
        Ckp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Chip select polarity for chip select 0 0- Active low 1- Active high"]
    #[inline(always)]
    pub fn csp0(&self) -> Csp0R {
        Csp0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clock phase for chip select 0. If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
    #[inline(always)]
    pub fn ckph0(&self) -> Ckph0R {
        Ckph0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Data delay for chip select 0 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
    #[inline(always)]
    pub fn dd0(&self) -> Dd0R {
        Dd0R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Clock polarity for chip select 1 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
    #[inline(always)]
    pub fn ckp1(&self) -> Ckp1R {
        Ckp1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Chip select polarity for chip select 1 0- Active low 1- Active high"]
    #[inline(always)]
    pub fn csp1(&self) -> Csp1R {
        Csp1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Clock phase for chip select 1. If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
    #[inline(always)]
    pub fn ckph1(&self) -> Ckph1R {
        Ckph1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Data delay for chip select 1 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
    #[inline(always)]
    pub fn dd1(&self) -> Dd1R {
        Dd1R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Clock polarity for chip select 2 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
    #[inline(always)]
    pub fn ckp2(&self) -> Ckp2R {
        Ckp2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Chip select polarity for chip select 2 0- Active low 1- Active high"]
    #[inline(always)]
    pub fn csp2(&self) -> Csp2R {
        Csp2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Clock phase for chip select 2. If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
    #[inline(always)]
    pub fn ckph2(&self) -> Ckph2R {
        Ckph2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Data delay for chip select 2 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
    #[inline(always)]
    pub fn dd2(&self) -> Dd2R {
        Dd2R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Clock polarity for chip select 3 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
    #[inline(always)]
    pub fn ckp3(&self) -> Ckp3R {
        Ckp3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Chip select polarity for chip select 3 0- Active low 1- Active high"]
    #[inline(always)]
    pub fn csp3(&self) -> Csp3R {
        Csp3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Clock phase for chip select 3 If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
    #[inline(always)]
    pub fn ckph3(&self) -> Ckph3R {
        Ckph3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Data delay for chip select 3 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
    #[inline(always)]
    pub fn dd3(&self) -> Dd3R {
        Dd3R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clock polarity for chip select 0 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
    #[inline(always)]
    #[must_use]
    pub fn ckp0(&mut self) -> Ckp0W<SpiDcSpec> {
        Ckp0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Chip select polarity for chip select 0 0- Active low 1- Active high"]
    #[inline(always)]
    #[must_use]
    pub fn csp0(&mut self) -> Csp0W<SpiDcSpec> {
        Csp0W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clock phase for chip select 0. If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn ckph0(&mut self) -> Ckph0W<SpiDcSpec> {
        Ckph0W::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Data delay for chip select 0 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
    #[inline(always)]
    #[must_use]
    pub fn dd0(&mut self) -> Dd0W<SpiDcSpec> {
        Dd0W::new(self, 3)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SpiDcSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
Clock polarity for chip select 1 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
    #[inline(always)]
    #[must_use]
    pub fn ckp1(&mut self) -> Ckp1W<SpiDcSpec> {
        Ckp1W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Chip select polarity for chip select 1 0- Active low 1- Active high"]
    #[inline(always)]
    #[must_use]
    pub fn csp1(&mut self) -> Csp1W<SpiDcSpec> {
        Csp1W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Clock phase for chip select 1. If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn ckph1(&mut self) -> Ckph1W<SpiDcSpec> {
        Ckph1W::new(self, 10)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Data delay for chip select 1 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
    #[inline(always)]
    #[must_use]
    pub fn dd1(&mut self) -> Dd1W<SpiDcSpec> {
        Dd1W::new(self, 11)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SpiDcSpec> {
        Reserved2W::new(self, 13)
    }
    #[doc = "Bit 16 - 16:16\\]
Clock polarity for chip select 2 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
    #[inline(always)]
    #[must_use]
    pub fn ckp2(&mut self) -> Ckp2W<SpiDcSpec> {
        Ckp2W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Chip select polarity for chip select 2 0- Active low 1- Active high"]
    #[inline(always)]
    #[must_use]
    pub fn csp2(&mut self) -> Csp2W<SpiDcSpec> {
        Csp2W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Clock phase for chip select 2. If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn ckph2(&mut self) -> Ckph2W<SpiDcSpec> {
        Ckph2W::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Data delay for chip select 2 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
    #[inline(always)]
    #[must_use]
    pub fn dd2(&mut self) -> Dd2W<SpiDcSpec> {
        Dd2W::new(self, 19)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SpiDcSpec> {
        Reserved3W::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Clock polarity for chip select 3 0- When data is not being transferred, SCK = 0 1- When data is not being transferred, SCK = 1"]
    #[inline(always)]
    #[must_use]
    pub fn ckp3(&mut self) -> Ckp3W<SpiDcSpec> {
        Ckp3W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Chip select polarity for chip select 3 0- Active low 1- Active high"]
    #[inline(always)]
    #[must_use]
    pub fn csp3(&mut self) -> Csp3W<SpiDcSpec> {
        Csp3W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Clock phase for chip select 3 If CKP0 = 0 0- Data shifted out on falling edge; input on rising edge 1- Data shifted out on rising edge; input on falling edge If CKP0 = 1 1- Data shifted out on falling edge; input on rising edge 0- Data shifted out on rising edge; input on falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn ckph3(&mut self) -> Ckph3W<SpiDcSpec> {
        Ckph3W::new(self, 26)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Data delay for chip select 3 00- Data is output on the same cycle as the CS_N goes active 01- Data is output 1 DCLK cycle after the CS_N goes active 10- Data is output 2 DCLK cycles after the CS_N goes active 11- Data is output 3 DCLK cycles after the CS_N goes active"]
    #[inline(always)]
    #[must_use]
    pub fn dd3(&mut self) -> Dd3W<SpiDcSpec> {
        Dd3W::new(self, 27)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<SpiDcSpec> {
        Reserved4W::new(self, 29)
    }
}
#[doc = "SPI Data Control Register (SPIDC)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiDcSpec;
impl crate::RegisterSpec for SpiDcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_dc::R`](R) reader structure"]
impl crate::Readable for SpiDcSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_dc::W`](W) writer structure"]
impl crate::Writable for SpiDcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_DC to value 0"]
impl crate::Resettable for SpiDcSpec {
    const RESET_VALUE: u32 = 0;
}
