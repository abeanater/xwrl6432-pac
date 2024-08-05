#[doc = "Register `CCCFG` reader"]
pub type R = crate::R<CccfgSpec>;
#[doc = "Register `CCCFG` writer"]
pub type W = crate::W<CccfgSpec>;
#[doc = "Field `NUMDMACH` reader - 2:0\\]
Number of DMA Channels"]
pub type NumdmachR = crate::FieldReader;
#[doc = "Field `NUMDMACH` writer - 2:0\\]
Number of DMA Channels"]
pub type NumdmachW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES8` reader - 3:3\\]
RESERVE FIELD"]
pub type Res8R = crate::BitReader;
#[doc = "Field `RES8` writer - 3:3\\]
RESERVE FIELD"]
pub type Res8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMQDMACH` reader - 6:4\\]
Number of QDMA Channels"]
pub type NumqdmachR = crate::FieldReader;
#[doc = "Field `NUMQDMACH` writer - 6:4\\]
Number of QDMA Channels"]
pub type NumqdmachW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES7` reader - 7:7\\]
RESERVE FIELD"]
pub type Res7R = crate::BitReader;
#[doc = "Field `RES7` writer - 7:7\\]
RESERVE FIELD"]
pub type Res7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMINTCH` reader - 10:8\\]
Number of Interrupt Channels"]
pub type NumintchR = crate::FieldReader;
#[doc = "Field `NUMINTCH` writer - 10:8\\]
Number of Interrupt Channels"]
pub type NumintchW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES6` reader - 11:11\\]
RESERVE FIELD"]
pub type Res6R = crate::BitReader;
#[doc = "Field `RES6` writer - 11:11\\]
RESERVE FIELD"]
pub type Res6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMPAENTRY` reader - 14:12\\]
Number of PaRAM entries"]
pub type NumpaentryR = crate::FieldReader;
#[doc = "Field `NUMPAENTRY` writer - 14:12\\]
Number of PaRAM entries"]
pub type NumpaentryW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES5` reader - 15:15\\]
RESERVE FIELD"]
pub type Res5R = crate::BitReader;
#[doc = "Field `RES5` writer - 15:15\\]
RESERVE FIELD"]
pub type Res5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMTC` reader - 18:16\\]
Number of Queues/Number of TCs"]
pub type NumtcR = crate::FieldReader;
#[doc = "Field `NUMTC` writer - 18:16\\]
Number of Queues/Number of TCs"]
pub type NumtcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES4` reader - 19:19\\]
RESERVE FIELD"]
pub type Res4R = crate::BitReader;
#[doc = "Field `RES4` writer - 19:19\\]
RESERVE FIELD"]
pub type Res4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMREGN` reader - 21:20\\]
Number of MP and Shadow regions"]
pub type NumregnR = crate::FieldReader;
#[doc = "Field `NUMREGN` writer - 21:20\\]
Number of MP and Shadow regions"]
pub type NumregnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES3` reader - 23:22\\]
RESERVE FIELD"]
pub type Res3R = crate::FieldReader;
#[doc = "Field `RES3` writer - 23:22\\]
RESERVE FIELD"]
pub type Res3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHMAPEXIST` reader - 24:24\\]
Channel Mapping Existence CHMAPEXIST = 0 : No Channel mapping. CHMAPEXIST = 1 : Channel mapping logic included."]
pub type ChmapexistR = crate::BitReader;
#[doc = "Field `CHMAPEXIST` writer - 24:24\\]
Channel Mapping Existence CHMAPEXIST = 0 : No Channel mapping. CHMAPEXIST = 1 : Channel mapping logic included."]
pub type ChmapexistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPEXIST` reader - 25:25\\]
Memory Protection Existence MPEXIST = 0 : No memory protection. MPEXIST = 1 : Memory Protection logic included."]
pub type MpexistR = crate::BitReader;
#[doc = "Field `MPEXIST` writer - 25:25\\]
Memory Protection Existence MPEXIST = 0 : No memory protection. MPEXIST = 1 : Memory Protection logic included."]
pub type MpexistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES2` reader - 31:26\\]
RESERVE FIELD"]
pub type Res2R = crate::FieldReader;
#[doc = "Field `RES2` writer - 31:26\\]
RESERVE FIELD"]
pub type Res2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Number of DMA Channels"]
    #[inline(always)]
    pub fn numdmach(&self) -> NumdmachR {
        NumdmachR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res8(&self) -> Res8R {
        Res8R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Number of QDMA Channels"]
    #[inline(always)]
    pub fn numqdmach(&self) -> NumqdmachR {
        NumqdmachR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res7(&self) -> Res7R {
        Res7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Number of Interrupt Channels"]
    #[inline(always)]
    pub fn numintch(&self) -> NumintchR {
        NumintchR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res6(&self) -> Res6R {
        Res6R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Number of PaRAM entries"]
    #[inline(always)]
    pub fn numpaentry(&self) -> NumpaentryR {
        NumpaentryR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res5(&self) -> Res5R {
        Res5R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Number of Queues/Number of TCs"]
    #[inline(always)]
    pub fn numtc(&self) -> NumtcR {
        NumtcR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res4(&self) -> Res4R {
        Res4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Number of MP and Shadow regions"]
    #[inline(always)]
    pub fn numregn(&self) -> NumregnR {
        NumregnR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res3(&self) -> Res3R {
        Res3R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Channel Mapping Existence CHMAPEXIST = 0 : No Channel mapping. CHMAPEXIST = 1 : Channel mapping logic included."]
    #[inline(always)]
    pub fn chmapexist(&self) -> ChmapexistR {
        ChmapexistR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Memory Protection Existence MPEXIST = 0 : No memory protection. MPEXIST = 1 : Memory Protection logic included."]
    #[inline(always)]
    pub fn mpexist(&self) -> MpexistR {
        MpexistR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res2(&self) -> Res2R {
        Res2R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Number of DMA Channels"]
    #[inline(always)]
    #[must_use]
    pub fn numdmach(&mut self) -> NumdmachW<CccfgSpec> {
        NumdmachW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res8(&mut self) -> Res8W<CccfgSpec> {
        Res8W::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Number of QDMA Channels"]
    #[inline(always)]
    #[must_use]
    pub fn numqdmach(&mut self) -> NumqdmachW<CccfgSpec> {
        NumqdmachW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res7(&mut self) -> Res7W<CccfgSpec> {
        Res7W::new(self, 7)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Number of Interrupt Channels"]
    #[inline(always)]
    #[must_use]
    pub fn numintch(&mut self) -> NumintchW<CccfgSpec> {
        NumintchW::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res6(&mut self) -> Res6W<CccfgSpec> {
        Res6W::new(self, 11)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Number of PaRAM entries"]
    #[inline(always)]
    #[must_use]
    pub fn numpaentry(&mut self) -> NumpaentryW<CccfgSpec> {
        NumpaentryW::new(self, 12)
    }
    #[doc = "Bit 15 - 15:15\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res5(&mut self) -> Res5W<CccfgSpec> {
        Res5W::new(self, 15)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Number of Queues/Number of TCs"]
    #[inline(always)]
    #[must_use]
    pub fn numtc(&mut self) -> NumtcW<CccfgSpec> {
        NumtcW::new(self, 16)
    }
    #[doc = "Bit 19 - 19:19\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res4(&mut self) -> Res4W<CccfgSpec> {
        Res4W::new(self, 19)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Number of MP and Shadow regions"]
    #[inline(always)]
    #[must_use]
    pub fn numregn(&mut self) -> NumregnW<CccfgSpec> {
        NumregnW::new(self, 20)
    }
    #[doc = "Bits 22:23 - 23:22\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res3(&mut self) -> Res3W<CccfgSpec> {
        Res3W::new(self, 22)
    }
    #[doc = "Bit 24 - 24:24\\]
Channel Mapping Existence CHMAPEXIST = 0 : No Channel mapping. CHMAPEXIST = 1 : Channel mapping logic included."]
    #[inline(always)]
    #[must_use]
    pub fn chmapexist(&mut self) -> ChmapexistW<CccfgSpec> {
        ChmapexistW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Memory Protection Existence MPEXIST = 0 : No memory protection. MPEXIST = 1 : Memory Protection logic included."]
    #[inline(always)]
    #[must_use]
    pub fn mpexist(&mut self) -> MpexistW<CccfgSpec> {
        MpexistW::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res2(&mut self) -> Res2W<CccfgSpec> {
        Res2W::new(self, 26)
    }
}
#[doc = "CC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccfgSpec;
impl crate::RegisterSpec for CccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccfg::R`](R) reader structure"]
impl crate::Readable for CccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cccfg::W`](W) writer structure"]
impl crate::Writable for CccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCFG to value 0"]
impl crate::Resettable for CccfgSpec {
    const RESET_VALUE: u32 = 0;
}
