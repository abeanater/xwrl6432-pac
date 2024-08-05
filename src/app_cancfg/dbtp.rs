#[doc = "Register `DBTP` reader"]
pub type R = crate::R<DbtpSpec>;
#[doc = "Register `DBTP` writer"]
pub type W = crate::W<DbtpSpec>;
#[doc = "Field `DSJW` reader - 3:0\\]
Data resynchronization Jump Width"]
pub type DsjwR = crate::FieldReader;
#[doc = "Field `DSJW` writer - 3:0\\]
Data resynchronization Jump Width"]
pub type DsjwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG2` reader - 7:4\\]
Data time segment after sample point"]
pub type Dtseg2R = crate::FieldReader;
#[doc = "Field `DTSEG2` writer - 7:4\\]
Data time segment after sample point"]
pub type Dtseg2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG1` reader - 12:8\\]
Data time segment before smaple point"]
pub type Dtseg1R = crate::FieldReader;
#[doc = "Field `DTSEG1` writer - 12:8\\]
Data time segment before smaple point"]
pub type Dtseg1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU11` reader - 15:13\\]
Reserved"]
pub type Nu11R = crate::FieldReader;
#[doc = "Field `NU11` writer - 15:13\\]
Reserved"]
pub type Nu11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBRP` reader - 20:16\\]
Data Baud Rate Prescaler"]
pub type DbrpR = crate::FieldReader;
#[doc = "Field `DBRP` writer - 20:16\\]
Data Baud Rate Prescaler"]
pub type DbrpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU12` reader - 22:21\\]
Reserved"]
pub type Nu12R = crate::FieldReader;
#[doc = "Field `NU12` writer - 22:21\\]
Reserved"]
pub type Nu12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TDC` reader - 23:23\\]
Transmitter Delay Compensation"]
pub type TdcR = crate::BitReader;
#[doc = "Field `TDC` writer - 23:23\\]
Transmitter Delay Compensation"]
pub type TdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU13` reader - 31:24\\]
Reserved"]
pub type Nu13R = crate::FieldReader;
#[doc = "Field `NU13` writer - 31:24\\]
Reserved"]
pub type Nu13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Data resynchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DsjwR {
        DsjwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg2(&self) -> Dtseg2R {
        Dtseg2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Data time segment before smaple point"]
    #[inline(always)]
    pub fn dtseg1(&self) -> Dtseg1R {
        Dtseg1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Reserved"]
    #[inline(always)]
    pub fn nu11(&self) -> Nu11R {
        Nu11R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Data Baud Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&self) -> DbrpR {
        DbrpR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Reserved"]
    #[inline(always)]
    pub fn nu12(&self) -> Nu12R {
        Nu12R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Transmitter Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TdcR {
        TdcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    pub fn nu13(&self) -> Nu13R {
        Nu13R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Data resynchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn dsjw(&mut self) -> DsjwW<DbtpSpec> {
        DsjwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Data time segment after sample point"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg2(&mut self) -> Dtseg2W<DbtpSpec> {
        Dtseg2W::new(self, 4)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Data time segment before smaple point"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg1(&mut self) -> Dtseg1W<DbtpSpec> {
        Dtseg1W::new(self, 8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu11(&mut self) -> Nu11W<DbtpSpec> {
        Nu11W::new(self, 13)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Data Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dbrp(&mut self) -> DbrpW<DbtpSpec> {
        DbrpW::new(self, 16)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu12(&mut self) -> Nu12W<DbtpSpec> {
        Nu12W::new(self, 21)
    }
    #[doc = "Bit 23 - 23:23\\]
Transmitter Delay Compensation"]
    #[inline(always)]
    #[must_use]
    pub fn tdc(&mut self) -> TdcW<DbtpSpec> {
        TdcW::new(self, 23)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu13(&mut self) -> Nu13W<DbtpSpec> {
        Nu13W::new(self, 24)
    }
}
#[doc = "DBTP\n\nYou can [`read`](crate::Reg::read) this register and get [`dbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbtpSpec;
impl crate::RegisterSpec for DbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbtp::R`](R) reader structure"]
impl crate::Readable for DbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`dbtp::W`](W) writer structure"]
impl crate::Writable for DbtpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBTP to value 0"]
impl crate::Resettable for DbtpSpec {
    const RESET_VALUE: u32 = 0;
}
