#[doc = "Register `REG12` reader"]
pub type R = crate::R<Reg12Spec>;
#[doc = "Register `REG12` writer"]
pub type W = crate::W<Reg12Spec>;
#[doc = "Field `DRAM_ECC_ENABLE` reader - "]
pub type DramEccEnableR = crate::BitReader;
#[doc = "Field `DRAM_ECC_ENABLE` writer - "]
pub type DramEccEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 7:1\\]
TI reserved"]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 7:1\\]
TI reserved"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DRAM_ECC_ERR_CLR` reader - 8:8\\]
TI reserved"]
pub type DramEccErrClrR = crate::BitReader;
#[doc = "Field `DRAM_ECC_ERR_CLR` writer - 8:8\\]
TI reserved"]
pub type DramEccErrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 15:9\\]
TI reserved"]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 15:9\\]
TI reserved"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DRAM_ECC_ERR_ADDR` reader - 23:16\\]
TI reserved"]
pub type DramEccErrAddrR = crate::FieldReader;
#[doc = "Field `DRAM_ECC_ERR_ADDR` writer - 23:16\\]
TI reserved"]
pub type DramEccErrAddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DRAM_REPAIRED_BIT` reader - 31:24\\]
TI reserved"]
pub type DramRepairedBitR = crate::FieldReader;
#[doc = "Field `DRAM_REPAIRED_BIT` writer - 31:24\\]
TI reserved"]
pub type DramRepairedBitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dram_ecc_enable(&self) -> DramEccEnableR {
        DramEccEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
TI reserved"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
TI reserved"]
    #[inline(always)]
    pub fn dram_ecc_err_clr(&self) -> DramEccErrClrR {
        DramEccErrClrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
TI reserved"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI reserved"]
    #[inline(always)]
    pub fn dram_ecc_err_addr(&self) -> DramEccErrAddrR {
        DramEccErrAddrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI reserved"]
    #[inline(always)]
    pub fn dram_repaired_bit(&self) -> DramRepairedBitR {
        DramRepairedBitR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dram_ecc_enable(&mut self) -> DramEccEnableW<Reg12Spec> {
        DramEccEnableW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Reg12Spec> {
        Nu1W::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dram_ecc_err_clr(&mut self) -> DramEccErrClrW<Reg12Spec> {
        DramEccErrClrW::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Reg12Spec> {
        Nu2W::new(self, 9)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dram_ecc_err_addr(&mut self) -> DramEccErrAddrW<Reg12Spec> {
        DramEccErrAddrW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dram_repaired_bit(&mut self) -> DramRepairedBitW<Reg12Spec> {
        DramRepairedBitW::new(self, 24)
    }
}
#[doc = "REG12\n\nYou can [`read`](crate::Reg::read) this register and get [`reg12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg12Spec;
impl crate::RegisterSpec for Reg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg12::R`](R) reader structure"]
impl crate::Readable for Reg12Spec {}
#[doc = "`write(|w| ..)` method takes [`reg12::W`](W) writer structure"]
impl crate::Writable for Reg12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG12 to value 0"]
impl crate::Resettable for Reg12Spec {
    const RESET_VALUE: u32 = 0;
}
