#[doc = "Register `CH3CTRL` reader"]
pub type R = crate::R<Ch3ctrlSpec>;
#[doc = "Register `CH3CTRL` writer"]
pub type W = crate::W<Ch3ctrlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Channel Enable - (RW )"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Channel Enable - (RW )"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED_1` reader - 7:1\\]
Read returns 0 - (RO )"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED_1` writer - 7:1\\]
Read returns 0 - (RO )"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EXTCLK` reader - 15:8\\]
Clock ratio extension: This register is used to concatenate with MCSPI_CHCONF\\[CLKD\\]
register for clock ratio only when granularity is one clock cycle \\[MCSPI_CHCONF\\[CLKG\\]
set to 1\\]
Then the max value reached is 4096 clock divider ratio - (RW )"]
pub type ExtclkR = crate::FieldReader;
#[doc = "Field `EXTCLK` writer - 15:8\\]
Clock ratio extension: This register is used to concatenate with MCSPI_CHCONF\\[CLKD\\]
register for clock ratio only when granularity is one clock cycle \\[MCSPI_CHCONF\\[CLKG\\]
set to 1\\]
Then the max value reached is 4096 clock divider ratio - (RW )"]
pub type ExtclkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED_2` reader - 31:16\\]
Read returns 0 - (RO )"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED_2` writer - 31:16\\]
Read returns 0 - (RO )"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel Enable - (RW )"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Read returns 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Clock ratio extension: This register is used to concatenate with MCSPI_CHCONF\\[CLKD\\]
register for clock ratio only when granularity is one clock cycle \\[MCSPI_CHCONF\\[CLKG\\]
set to 1\\]
Then the max value reached is 4096 clock divider ratio - (RW )"]
    #[inline(always)]
    pub fn extclk(&self) -> ExtclkR {
        ExtclkR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Read returns 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel Enable - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Ch3ctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Read returns 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_1(&mut self) -> Reserved1W<Ch3ctrlSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Clock ratio extension: This register is used to concatenate with MCSPI_CHCONF\\[CLKD\\]
register for clock ratio only when granularity is one clock cycle \\[MCSPI_CHCONF\\[CLKG\\]
set to 1\\]
Then the max value reached is 4096 clock divider ratio - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn extclk(&mut self) -> ExtclkW<Ch3ctrlSpec> {
        ExtclkW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Read returns 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_2(&mut self) -> Reserved2W<Ch3ctrlSpec> {
        Reserved2W::new(self, 16)
    }
}
#[doc = "This register is dedicated to enable the channel 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3ctrlSpec;
impl crate::RegisterSpec for Ch3ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3ctrl::R`](R) reader structure"]
impl crate::Readable for Ch3ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3ctrl::W`](W) writer structure"]
impl crate::Writable for Ch3ctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH3CTRL to value 0"]
impl crate::Resettable for Ch3ctrlSpec {
    const RESET_VALUE: u32 = 0;
}
