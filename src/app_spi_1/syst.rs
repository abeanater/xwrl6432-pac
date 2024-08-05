#[doc = "Register `SYST` reader"]
pub type R = crate::R<SystSpec>;
#[doc = "Register `SYST` writer"]
pub type W = crate::W<SystSpec>;
#[doc = "Field `SPIEN_0` reader - 0:0\\]
SPIEN\\[0\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[0\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[0\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
pub type Spien0R = crate::BitReader;
#[doc = "Field `SPIEN_0` writer - 0:0\\]
SPIEN\\[0\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[0\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[0\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
pub type Spien0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIEN_1` reader - 1:1\\]
SPIEN\\[1\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[1\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[1\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
pub type Spien1R = crate::BitReader;
#[doc = "Field `SPIEN_1` writer - 1:1\\]
SPIEN\\[1\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[1\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[1\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
pub type Spien1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIEN_2` reader - 2:2\\]
SPIEN\\[2\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[2\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[2\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
pub type Spien2R = crate::BitReader;
#[doc = "Field `SPIEN_2` writer - 2:2\\]
SPIEN\\[2\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[2\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[2\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
pub type Spien2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIEN_3` reader - 3:3\\]
SPIEN\\[3\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[3\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[3\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
pub type Spien3R = crate::BitReader;
#[doc = "Field `SPIEN_3` writer - 3:3\\]
SPIEN\\[3\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[3\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[3\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
pub type Spien3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIDAT_0` reader - 4:4\\]
SPIDAT\\[0\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIDATDIR0\\]
= 0 \\[output mode direction\\]
the SPIDAT\\[0\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIDATDIR0\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIDAT\\[0\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
pub type Spidat0R = crate::BitReader;
#[doc = "Field `SPIDAT_0` writer - 4:4\\]
SPIDAT\\[0\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIDATDIR0\\]
= 0 \\[output mode direction\\]
the SPIDAT\\[0\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIDATDIR0\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIDAT\\[0\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
pub type Spidat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIDAT_1` reader - 5:5\\]
SPIDAT\\[1\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIDATDIR1\\]
= 0 \\[output mode direction\\]
the SPIDAT\\[1\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIDATDIR1\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIDAT\\[1\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
pub type Spidat1R = crate::BitReader;
#[doc = "Field `SPIDAT_1` writer - 5:5\\]
SPIDAT\\[1\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIDATDIR1\\]
= 0 \\[output mode direction\\]
the SPIDAT\\[1\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIDATDIR1\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIDAT\\[1\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
pub type Spidat1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPICLK` reader - 6:6\\]
SPICLK line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the CLKSPI line \\[high or low\\]
and a write into this bit has no effect If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the CLKSPI line is driven high or low according to the value written into this register - (RW )"]
pub type SpiclkR = crate::BitReader;
#[doc = "Field `SPICLK` writer - 6:6\\]
SPICLK line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the CLKSPI line \\[high or low\\]
and a write into this bit has no effect If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the CLKSPI line is driven high or low according to the value written into this register - (RW )"]
pub type SpiclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKD` reader - 7:7\\]
SWAKEUP output \\[signal data value of internal signal to system\\]
The signal is driven high or low according to the value written into this register bit - (RW )"]
pub type WakdR = crate::BitReader;
#[doc = "Field `WAKD` writer - 7:7\\]
SWAKEUP output \\[signal data value of internal signal to system\\]
The signal is driven high or low according to the value written into this register bit - (RW )"]
pub type WakdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIDATDIR0` reader - 8:8\\]
Set the direction of the SPIDAT\\[0\\]
- (RW )"]
pub type Spidatdir0R = crate::BitReader;
#[doc = "Field `SPIDATDIR0` writer - 8:8\\]
Set the direction of the SPIDAT\\[0\\]
- (RW )"]
pub type Spidatdir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIDATDIR1` reader - 9:9\\]
Set the direction of the SPIDAT\\[1\\]
- (RW )"]
pub type Spidatdir1R = crate::BitReader;
#[doc = "Field `SPIDATDIR1` writer - 9:9\\]
Set the direction of the SPIDAT\\[1\\]
- (RW )"]
pub type Spidatdir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIENDIR` reader - 10:10\\]
Set the direction of the SPIEN\\[3:0\\]
lines and SPICLK line - (RW )"]
pub type SpiendirR = crate::BitReader;
#[doc = "Field `SPIENDIR` writer - 10:10\\]
Set the direction of the SPIEN\\[3:0\\]
lines and SPICLK line - (RW )"]
pub type SpiendirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSB` reader - 11:11\\]
Set status bit - (RW )"]
pub type SsbR = crate::BitReader;
#[doc = "Field `SSB` writer - 11:11\\]
Set status bit - (RW )"]
pub type SsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED_17` reader - 31:12\\]
Reads returns 0 - (RO )"]
pub type Reserved17R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_17` writer - 31:12\\]
Reads returns 0 - (RO )"]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SPIEN\\[0\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[0\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[0\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
    #[inline(always)]
    pub fn spien_0(&self) -> Spien0R {
        Spien0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SPIEN\\[1\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[1\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[1\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
    #[inline(always)]
    pub fn spien_1(&self) -> Spien1R {
        Spien1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
SPIEN\\[2\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[2\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[2\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
    #[inline(always)]
    pub fn spien_2(&self) -> Spien2R {
        Spien2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
SPIEN\\[3\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[3\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[3\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
    #[inline(always)]
    pub fn spien_3(&self) -> Spien3R {
        Spien3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
SPIDAT\\[0\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIDATDIR0\\]
= 0 \\[output mode direction\\]
the SPIDAT\\[0\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIDATDIR0\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIDAT\\[0\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
    #[inline(always)]
    pub fn spidat_0(&self) -> Spidat0R {
        Spidat0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
SPIDAT\\[1\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIDATDIR1\\]
= 0 \\[output mode direction\\]
the SPIDAT\\[1\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIDATDIR1\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIDAT\\[1\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
    #[inline(always)]
    pub fn spidat_1(&self) -> Spidat1R {
        Spidat1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SPICLK line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the CLKSPI line \\[high or low\\]
and a write into this bit has no effect If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the CLKSPI line is driven high or low according to the value written into this register - (RW )"]
    #[inline(always)]
    pub fn spiclk(&self) -> SpiclkR {
        SpiclkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
SWAKEUP output \\[signal data value of internal signal to system\\]
The signal is driven high or low according to the value written into this register bit - (RW )"]
    #[inline(always)]
    pub fn wakd(&self) -> WakdR {
        WakdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Set the direction of the SPIDAT\\[0\\]
- (RW )"]
    #[inline(always)]
    pub fn spidatdir0(&self) -> Spidatdir0R {
        Spidatdir0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Set the direction of the SPIDAT\\[1\\]
- (RW )"]
    #[inline(always)]
    pub fn spidatdir1(&self) -> Spidatdir1R {
        Spidatdir1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Set the direction of the SPIEN\\[3:0\\]
lines and SPICLK line - (RW )"]
    #[inline(always)]
    pub fn spiendir(&self) -> SpiendirR {
        SpiendirR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Set status bit - (RW )"]
    #[inline(always)]
    pub fn ssb(&self) -> SsbR {
        SsbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reads returns 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_17(&self) -> Reserved17R {
        Reserved17R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SPIEN\\[0\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[0\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[0\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn spien_0(&mut self) -> Spien0W<SystSpec> {
        Spien0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SPIEN\\[1\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[1\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[1\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn spien_1(&mut self) -> Spien1W<SystSpec> {
        Spien1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
SPIEN\\[2\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[2\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[2\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn spien_2(&mut self) -> Spien2W<SystSpec> {
        Spien2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
SPIEN\\[3\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the SPIENT\\[3\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIEN\\[3\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn spien_3(&mut self) -> Spien3W<SystSpec> {
        Spien3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
SPIDAT\\[0\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIDATDIR0\\]
= 0 \\[output mode direction\\]
the SPIDAT\\[0\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIDATDIR0\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIDAT\\[0\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn spidat_0(&mut self) -> Spidat0W<SystSpec> {
        Spidat0W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
SPIDAT\\[1\\]
line \\[signal data value\\]
If MCSPI_SYST\\[SPIDATDIR1\\]
= 0 \\[output mode direction\\]
the SPIDAT\\[1\\]
line is driven high or low according to the value written into this register If MCSPI_SYST\\[SPIDATDIR1\\]
= 1 \\[input mode direction\\]
this bit returns the value on the SPIDAT\\[1\\]
line \\[high or low\\]
and a write into this bit has no effect - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn spidat_1(&mut self) -> Spidat1W<SystSpec> {
        Spidat1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
SPICLK line \\[signal data value\\]
If MCSPI_SYST\\[SPIENDIR\\]
= 1 \\[input mode direction\\]
this bit returns the value on the CLKSPI line \\[high or low\\]
and a write into this bit has no effect If MCSPI_SYST\\[SPIENDIR\\]
= 0 \\[output mode direction\\]
the CLKSPI line is driven high or low according to the value written into this register - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn spiclk(&mut self) -> SpiclkW<SystSpec> {
        SpiclkW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
SWAKEUP output \\[signal data value of internal signal to system\\]
The signal is driven high or low according to the value written into this register bit - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn wakd(&mut self) -> WakdW<SystSpec> {
        WakdW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Set the direction of the SPIDAT\\[0\\]
- (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn spidatdir0(&mut self) -> Spidatdir0W<SystSpec> {
        Spidatdir0W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Set the direction of the SPIDAT\\[1\\]
- (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn spidatdir1(&mut self) -> Spidatdir1W<SystSpec> {
        Spidatdir1W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Set the direction of the SPIEN\\[3:0\\]
lines and SPICLK line - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn spiendir(&mut self) -> SpiendirW<SystSpec> {
        SpiendirW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Set status bit - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn ssb(&mut self) -> SsbW<SystSpec> {
        SsbW::new(self, 11)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reads returns 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_17(&mut self) -> Reserved17W<SystSpec> {
        Reserved17W::new(self, 12)
    }
}
#[doc = "This register is used to check the correctness of the system interconnect either internally to peripheral bus or externally to device IO pads when the module is configured in system test (SYSTEST) mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`syst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystSpec;
impl crate::RegisterSpec for SystSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst::R`](R) reader structure"]
impl crate::Readable for SystSpec {}
#[doc = "`write(|w| ..)` method takes [`syst::W`](W) writer structure"]
impl crate::Writable for SystSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYST to value 0"]
impl crate::Resettable for SystSpec {
    const RESET_VALUE: u32 = 0;
}
