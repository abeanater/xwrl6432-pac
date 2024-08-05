#[doc = "Register `PLLDIG_MDIV_NDIV` reader"]
pub type R = crate::R<PlldigMdivNdivSpec>;
#[doc = "Register `PLLDIG_MDIV_NDIV` writer"]
pub type W = crate::W<PlldigMdivNdivSpec>;
#[doc = "Field `cfg_plldig_mdiv` reader - 8:0\\]
MDIV value for the PLL DIG Feedback divider settings. MDIV value directly programs the 9-bit feedback divider. Divide value ranges from 2 to 511. MDIV value has to be chosen to generate the required clock out frequency from the 2Mhz internal PLL reference"]
pub type CfgPlldigMdivR = crate::FieldReader<u16>;
#[doc = "Field `cfg_plldig_mdiv` writer - 8:0\\]
MDIV value for the PLL DIG Feedback divider settings. MDIV value directly programs the 9-bit feedback divider. Divide value ranges from 2 to 511. MDIV value has to be chosen to generate the required clock out frequency from the 2Mhz internal PLL reference"]
pub type CfgPlldigMdivW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `cfg_plldig_ndiv` reader - 22:16\\]
NDIV value for the PLL DIG Input clock divider settings .NDIV value directly programs the 7-bit pre- divider. Divide value ranges from 2 to 127. NDIV value has to be chosen based on the REF_CLKIN frequency so as to get the internal reference frequency of the PLL closest to 2Mhz"]
pub type CfgPlldigNdivR = crate::FieldReader;
#[doc = "Field `cfg_plldig_ndiv` writer - 22:16\\]
NDIV value for the PLL DIG Input clock divider settings .NDIV value directly programs the 7-bit pre- divider. Divide value ranges from 2 to 127. NDIV value has to be chosen based on the REF_CLKIN frequency so as to get the internal reference frequency of the PLL closest to 2Mhz"]
pub type CfgPlldigNdivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
MDIV value for the PLL DIG Feedback divider settings. MDIV value directly programs the 9-bit feedback divider. Divide value ranges from 2 to 511. MDIV value has to be chosen to generate the required clock out frequency from the 2Mhz internal PLL reference"]
    #[inline(always)]
    pub fn cfg_plldig_mdiv(&self) -> CfgPlldigMdivR {
        CfgPlldigMdivR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:22 - 22:16\\]
NDIV value for the PLL DIG Input clock divider settings .NDIV value directly programs the 7-bit pre- divider. Divide value ranges from 2 to 127. NDIV value has to be chosen based on the REF_CLKIN frequency so as to get the internal reference frequency of the PLL closest to 2Mhz"]
    #[inline(always)]
    pub fn cfg_plldig_ndiv(&self) -> CfgPlldigNdivR {
        CfgPlldigNdivR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
MDIV value for the PLL DIG Feedback divider settings. MDIV value directly programs the 9-bit feedback divider. Divide value ranges from 2 to 511. MDIV value has to be chosen to generate the required clock out frequency from the 2Mhz internal PLL reference"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_plldig_mdiv(&mut self) -> CfgPlldigMdivW<PlldigMdivNdivSpec> {
        CfgPlldigMdivW::new(self, 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
NDIV value for the PLL DIG Input clock divider settings .NDIV value directly programs the 7-bit pre- divider. Divide value ranges from 2 to 127. NDIV value has to be chosen based on the REF_CLKIN frequency so as to get the internal reference frequency of the PLL closest to 2Mhz"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_plldig_ndiv(&mut self) -> CfgPlldigNdivW<PlldigMdivNdivSpec> {
        CfgPlldigNdivW::new(self, 16)
    }
}
#[doc = "PLLDIG_MDIV_NDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_mdiv_ndiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_mdiv_ndiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlldigMdivNdivSpec;
impl crate::RegisterSpec for PlldigMdivNdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plldig_mdiv_ndiv::R`](R) reader structure"]
impl crate::Readable for PlldigMdivNdivSpec {}
#[doc = "`write(|w| ..)` method takes [`plldig_mdiv_ndiv::W`](W) writer structure"]
impl crate::Writable for PlldigMdivNdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLDIG_MDIV_NDIV to value 0"]
impl crate::Resettable for PlldigMdivNdivSpec {
    const RESET_VALUE: u32 = 0;
}
